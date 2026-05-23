# Architecture

> It is recommended to use dark themes

## 1. Overall Architecture

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}

  Core:::core

  Modules:::modules

  Messages:::messages

  Messages --> Modules --> Core
```
