# Process

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph BT
  {{#include ../mermaid-style}}
  
  ACommand[AsyncCommand]
  
  AChild[AsyncChild]
  
  AIsTerminal{AsyncIsTerminal}:::interface

```