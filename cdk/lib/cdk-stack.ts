import * as path from "node:path";
import * as cdk from "aws-cdk-lib";
import { RustFunction } from "cargo-lambda-cdk";
import type { Construct } from "constructs";
import * as apiGw from "aws-cdk-lib/aws-apigateway";

export class CdkStack extends cdk.Stack {
	constructor(scope: Construct, id: string, props?: cdk.StackProps) {
		super(scope, id, props);

		const rustLambdaFunction = new RustFunction(this, "RustFunction", {
			functionName: "RustFunctionForAxumTest",
			manifestPath: path.join(__dirname, "../../lambda"),
			runtime: "provided.al2023",
		});

		new apiGw.LambdaRestApi(this, "MyAPI", {
			handler: rustLambdaFunction,
		});
	}
}
