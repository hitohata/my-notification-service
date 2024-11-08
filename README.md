# My Notification Service

This is a service that handles notifications from my applications.

![overview](./img/aws-service-overview.drawio.svg)

This project exposes the IAM credential, which is used to execute put-events for the EventBridge, to the Parameter Store.

The producer of the messages must take this policy.
The incoming events are forwarded to the lambda and sent to the external third-party applications.

## To share policy

To share the policy among multiple region the lambda is used.
When deploy the resource by the CDK, the lambda will run.
The lambda get a parameter that is registered when deployment, then put parameters to multiple regions.
The regions are specified by GitHub's environment variable.

## Expected Schema

```json
{
  "application": "Notification",
  "notificationType": "Notification",
  "title": "title",
  "content": "content"
}
```

| Attribute        | Description           |
|:-----------------|:----------------------|
| application      | Application name      |
| notificationType | constant[^1]          | 
| title            | title of notification | 
| content          | content of message    |

[^1]: `NOTIFICATION`, `ERROR`