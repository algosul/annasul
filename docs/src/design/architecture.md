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
  
  I18n:::core

  Modules:::modules

  Messages:::messages
  
  CPU:::api
  GPU:::api
  NPU:::api
  
  UI:::ui
  AI:::ai

  AI --> NPU & GPU & CPU --> Messages
  UI --> GPU & CPU --> Messages

  Messages --> Modules --> I18n --> Core
```

## UI

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}

  UI:::ui
  
  UIFramework[UI Framework]:::ui
  UIBackend{UI Backend}:::interface
  
  DefaultUIBackend[Default UI Backend]:::ui
  
  RenderPipeline[Render Pipeline]:::ui

  RenderPipelineInterface{Render Pipeline Interface}:::interface
  
  DefaultUIBackend --> UIBackend
  DefaultUIBackend --> RenderPipeline 
  UI --> UIBackend & UIFramework
  
  RenderPipeline --> RenderPipelineInterface

```

### Backends

+ `algosul::cpu`
+ `algosul::gpu`
+ `Qt 6`
+ `Gtk 4`
+ `WinUI 2`

## AI

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}

  AI:::ai
  
  AIInference[AI Inference]:::ai
  
  AIInferenceInterface{AI Inference Interface}:::interface

  AI --> AIInference
  
  AIInference --> AIInferenceInterface
  
```

## Render

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}
  
  CPURenderPipeline[CPU Render Pipeline]:::ui
  GPURenderPipeline[GPU Render Pipeline]:::ui
  
  RenderPipelineInterface{Render Pipeline Interface}:::interface
  
  CPU:::api
  GPU:::api
  
  CPURenderPipeline & GPURenderPipeline --> RenderPipelineInterface
  
  CPURenderPipeline ---> CPU
  GPURenderPipeline ---> GPU
```

## AI Inference

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}
  
  CPUAIInference[CPU AI Inference]:::ai
  GPUAIInference[GPU AI Inference]:::ai
  
  AIInferenceInterface{AI Inference Interface}:::interface
  
  CPU:::api
  GPU:::api
  
  CPUAIInference & GPUAIInference --> AIInferenceInterface 
  
  CPUAIInference ---> CPU 
  GPUAIInference ---> GPU
```

## CPU

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}
  
  CPU:::api
  
  Task{Task System}:::interface
  
  CPU -..-> Task
  
```

## GPU

```mermaid
%%{
  init: {
    'theme': 'redux dark', 
  }
}%%
graph TB
  {{#include mermaid-style}}
  
  GPU:::api
  
  Cuda:::api
  Vulkan:::api
  DirectX:::api
  Task{Task System}:::interface
  
  GPU  --> Vulkan & DirectX & Cuda
  GPU -..-> Task
  
  Vulkan & DirectX -.-> Cuda
```

