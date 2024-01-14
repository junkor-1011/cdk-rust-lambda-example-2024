import * as cdk from 'aws-cdk-lib';
import { aws_apigateway as apigateway } from 'aws-cdk-lib';
import { RustFunction } from 'cargo-lambda-cdk';
import { Construct } from 'constructs';

export class CdkAppStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    new RustFunction(this, 'hello-world', {
      manifestPath: 'rs-lambda/hello-world/Cargo.toml',
      timeout: cdk.Duration.seconds(10),
      memorySize: 256,
    });

    const greetFunction = new RustFunction(this, 'greet', {
      manifestPath: 'rs-lambda/greet/Cargo.toml',
      timeout: cdk.Duration.seconds(10),
      memorySize: 256,
    });

    const greetApi = new apigateway.RestApi(this, 'Greet API', {
      restApiName: 'Greet API for test',
    });

    const greet = greetApi.root.addResource('greet');

    greet.addMethod('POST', new apigateway.LambdaIntegration(greetFunction));
  }
}
