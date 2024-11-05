import * as path from "node:path";
import * as cdk from "aws-cdk-lib";
import { RustFunction } from "cargo-lambda-cdk";
import type { Construct } from "constructs";

export class MyNotification extends cdk.Stack {
	constructor(scope: Construct, id: string, props?: cdk.StackProps) {
		super(scope, id, props);

		new RustFunction(this, "RustFunction", {
			functionName: "my-notification-function",
			manifestPath: path.join(__dirname, "../../lambda"),
			runtime: "provided.al2023",
		});

	}
}
