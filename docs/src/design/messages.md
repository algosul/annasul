# Messages

## Message Type

Basic type, does not support sending:

+ `Debug`
+ `Info`
+ `Warn`
+ `Error`

To define message types, you must choose to inherit from a message type

## Message Content

Needs to implement [Message Trait](messages/message_trait.md)

## Message Metadata

+ Status (`status`):
  + `Succeed`
  + `Failed`
+ Process (`process`): 0% ~ 100%
  + Progress detail (`progress_detail`)：`0/32`, `loading...`

## Message Sender

+ [Message Type](#message-type)
+ [Message Content](#message-content)
+ Sending time
+ Private ID of the message sending module (`sending_module_key`):
  Needs [Module Private ID](./modules.md#module-private-id)
+ [Message Metadata](#message-metadata)

## Message

+ [Message Type](#message-type)
+ [Message Content](#message-content)
+ Sending time
+ ID of the message sending module (`sending_module_id`): [Module ID](./modules.md#module-id)
+ [Message Metadata](#message-metadata)

## Message interface

+ Send message function
+ Register message type functions
+ Register the message hook function
