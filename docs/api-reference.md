# Galactic Neural Network API Reference

## Overview
This document provides comprehensive documentation for the Galactic Neural Network (GNN) API. The API enables developers to interact with the GNN platform, manage neural networks, and execute computations on the blockchain.

## API Endpoints

### Network Management

#### Initialize Network
```typescript
POST /api/v1/network/initialize

Request:
{
    "authority": "PublicKey",
    "initial_parameters": {
        "model_type": "string",
        "configuration": Object
    }
}

Response:
{
    "network_id": "string",
    "status": "string",
    "timestamp": "number"
}
```

#### Update Parameters
```typescript
PUT /api/v1/network/{network_id}/parameters

Request:
{
    "new_parameters": {
        "version": "number",
        "data": Uint8Array
    },
    "signature": "string"
}

Response:
{
    "status": "string",
    "version": "number",
    "timestamp": "number"
}
```

#### Execute Computation
```typescript
POST /api/v1/network/{network_id}/compute

Request:
{
    "input_data": Uint8Array,
    "computation_type": "string",
    "options": {
        "timeout": "number",
        "priority": "string"
    }
}

Response:
{
    "computation_id": "string",
    "status": "string",
    "result": Uint8Array
}
```

### Account Management

#### Create Account
```typescript
POST /api/v1/accounts/create

Request:
{
    "account_type": "string",
    "permissions": string[]
}

Response:
{
    "account_id": "string",
    "public_key": "string",
    "created_at": "number"
}
```

#### Get Account Info
```typescript
GET /api/v1/accounts/{account_id}

Response:
{
    "account_id": "string",
    "public_key": "string",
    "account_type": "string",
    "permissions": string[],
    "created_at": "number",
    "last_active": "number"
}
```

### Model Management

#### Deploy Model
```typescript
POST /api/v1/models/deploy

Request:
{
    "model_data": {
        "name": "string",
        "version": "string",
        "parameters": Uint8Array,
        "metadata": Object
    }
}

Response:
{
    "model_id": "string",
    "status": "string",
    "deployment_timestamp": "number"
}
```

#### Get Model Info
```typescript
GET /api/v1/models/{model_id}

Response:
{
    "model_id": "string",
    "name": "string",
    "version": "string",
    "status": "string",
    "metadata": Object,
    "created_at": "number",
    "last_updated": "number"
}
```

## WebSocket API

### Real-time Updates
```typescript
WS /ws/v1/network/{network_id}

// Subscribe to network updates
{
    "type": "subscribe",
    "channels": ["computation", "parameters", "status"]
}

// Computation update event
{
    "type": "computation_update",
    "data": {
        "computation_id": "string",
        "status": "string",
        "progress": "number",
        "timestamp": "number"
    }
}

// Parameter update event
{
    "type": "parameter_update",
    "data": {
        "version": "number",
        "timestamp": "number"
    }
}
```

## Error Codes

### HTTP Status Codes
- 200: Success
- 201: Created
- 400: Bad Request
- 401: Unauthorized
- 403: Forbidden
- 404: Not Found
- 500: Internal Server Error
- 503: Service Unavailable

### Custom Error Codes
```typescript
{
    "GNN-001": "Network initialization failed",
    "GNN-002": "Invalid parameters",
    "GNN-003": "Computation error",
    "GNN-004": "Model deployment failed",
    "GNN-005": "Insufficient permissions",
    "GNN-006": "Rate limit exceeded",
    "GNN-007": "Invalid signature",
    "GNN-008": "Network not found",
    "GNN-009": "Model not found"
}
```

## Data Structures

### NetworkState
```rust
pub struct NetworkState {
    pub is_initialized: bool,
    pub authority: Pubkey,
    pub model_version: u64,
    pub last_update: i64,
}
```

### ComputationResult
```typescript
interface ComputationResult {
    id: string;
    status: "pending" | "processing" | "completed" | "failed";
    result?: Uint8Array;
    error?: string;
    timestamp: number;
}
```

### ModelMetadata
```typescript
interface ModelMetadata {
    name: string;
    version: string;
    description?: string;
    parameters: {
        input_shape: number[];
        output_shape: number[];
        layer_config: Object;
    };
    performance_metrics?: {
        accuracy: number;
        latency: number;
        throughput: number;
    };
}
```

## Rate Limits

| Endpoint | Rate Limit |
|----------|------------|
| Network Initialization | 10 per hour |
| Parameter Updates | 100 per hour |
| Computations | 1000 per hour |
| Model Deployments | 50 per day |

## Authentication

### API Key Authentication
```typescript
Headers:
{
    "X-GNN-API-Key": "your-api-key"
}
```

### JWT Authentication
```typescript
Headers:
{
    "Authorization": "Bearer your-jwt-token"
}
```

## Examples

### JavaScript/TypeScript
```typescript
import { GNNClient } from '@galactic/gnn-client';

const client = new GNNClient({
    apiKey: 'your-api-key',
    endpoint: 'https://api.galacticnetwork.io'
});

// Initialize network
const network = await client.initializeNetwork({
    authority: publicKey,
    initial_parameters: {
        model_type: 'transformer',
        configuration: {
            layers: 12,
            heads: 8,
            hidden_size: 768
        }
    }
});

// Execute computation
const result = await client.executeComputation(network.network_id, {
    input_data: new Uint8Array([...]),
    computation_type: 'inference',
    options: {
        timeout: 5000,
        priority: 'high'
    }
});
```

### Python
```python
from galactic.gnn import GNNClient

client = GNNClient(
    api_key='your-api-key',
    endpoint='https://api.galacticnetwork.io'
)

# Deploy model
model = client.deploy_model({
    'name': 'my-model',
    'version': '1.0.0',
    'parameters': bytes([...]),
    'metadata': {
        'input_shape': [1, 768],
        'output_shape': [1, 10]
    }
})

# Get model info
model_info = client.get_model_info(model.model_id)
```

## Best Practices

1. Error Handling
```typescript
try {
    const result = await client.executeComputation(...);
} catch (error) {
    if (error.code === 'GNN-003') {
        // Handle computation error
    } else if (error.code === 'GNN-006') {
        // Handle rate limit
    }
}
```

2. Batch Operations
```typescript
const batch_results = await client.batchCompute([
    { input_data: data1, options: opts1 },
    { input_data: data2, options: opts2 }
]);
```

3. WebSocket Connection Management
```typescript
const ws = client.createWebSocket(network_id);

ws.on('open', () => {
    ws.subscribe(['computation', 'parameters']);
});

ws.on('computation_update', (update) => {
    console.log('Computation progress:', update.progress);
});

ws.on('error', (error) => {
    console.error('WebSocket error:', error);
    ws.reconnect();
});
```

## Support

For API support and questions:
- Email: api-support@galacticnetwork.io
- Documentation: https://docs.galacticnetwork.io
- Status Page: https://status.galacticnetwork.io 