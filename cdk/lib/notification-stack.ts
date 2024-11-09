import * as path from "node:path";
import * as cdk from "aws-cdk-lib";
import { CustomResource, aws_events_targets } from "aws-cdk-lib";
import * as events from "aws-cdk-lib/aws-events";
import { Effect, PolicyStatement } from "aws-cdk-lib/aws-iam";
import { StringParameter } from "aws-cdk-lib/aws-ssm";
import { Provider } from "aws-cdk-lib/custom-resources";
import { RustFunction } from "cargo-lambda-cdk";
import type { Construct } from "constructs";

const TARGET_REGION = process.env.TARGET_REGIONS;
const PARAMETER_NAME = "/arn/notification/event-bus";

export class MyNotification extends cdk.Stack {
	constructor(scope: Construct, id: string, props?: cdk.StackProps) {
		super(scope, id, props);

		// EventBus
		const notificationEventBus = new events.EventBus(
			this,
			"NotificationEventBus",
			{
				eventBusName: "MyNotificationBus",
				description: "Notification event bus",
			},
		);

		// set an event bus name to parameter store
		new StringParameter(this, "EventBusArmParameter", {
			parameterName: PARAMETER_NAME,
			stringValue: notificationEventBus.eventBusArn,
		});

		const notificationServiceFunction = new RustFunction(
			this,
			"NotificationFunction",
			{
				functionName: "my-notification-function",
				manifestPath: path.join(__dirname, "../../lambdas/notification"),
				runtime: "provided.al2023",
				description: "Notification service",
			},
		);

		new events.Rule(this, "MyNotificationServiceRule", {
			eventPattern: {
				account: [this.account]
			},
			eventBus: notificationEventBus,
			targets: [
				new aws_events_targets.LambdaFunction(notificationServiceFunction),
			],
		});

		this.putParameterSettings();
	}

	private putParameterSettings() {
		if (!TARGET_REGION) {
			throw Error("Not target");
		}

		const targetRegions: string[] = JSON.parse(TARGET_REGION);

		const putParametersFunction = new RustFunction(
			this,
			"PutParametersFunction",
			{
				functionName: "put-notification-parameters-service-function",
				manifestPath: path.join(__dirname, "../../lambdas/put_parameters"),
				runtime: "provided.al2023",
				description: "put parameters",
				environment: {
					TARGET_REGIONS: TARGET_REGION,
					ORIGIN: this.region,
					PARAMETER_NAME: PARAMETER_NAME,
				},
			},
		);

		putParametersFunction.addToRolePolicy(
			new PolicyStatement(
				new PolicyStatement({
					effect: Effect.ALLOW,
					actions: ["ssm:GetParameter"],
					resources: [
						`arn:aws:ssm:${this.region}:${this.account}:parameter${PARAMETER_NAME}`,
					],
				}),
			),
		);
		putParametersFunction.addToRolePolicy(
			new PolicyStatement(
				new PolicyStatement({
					effect: Effect.ALLOW,
					actions: ["ssm:PutParameter"],
					resources: targetRegions.map(
						(region) =>
							`arn:aws:ssm:${region}:${this.account}:parameter${PARAMETER_NAME}`,
					),
				}),
			),
		);

		const provider = new Provider(this, "custom-resource-provider", {
			onEventHandler: putParametersFunction,
		});

		new CustomResource(this, "custom-lambda-resource", {
			serviceToken: provider.serviceToken,
			properties: {
				uniqueId: Math.random(),
			},
		});
	}
}
