import * as cdk from 'aws-cdk-lib';
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
  }
}
