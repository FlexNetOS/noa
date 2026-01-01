# floodsub Protocol (Deprecated)

Simple pub/sub flooding.

**Location**: `p2p/protocols/floodsub/`  
**Version**: 0.47.0  
**Crate**: `libp2p-floodsub`

## ⚠️ Deprecated

Use [gossipsub](gossipsub.md) instead for production.

## Overview

Simple publish/subscribe with message flooding:

- Topic-based messaging
- No mesh optimization
- Full message flood

## NOA Usage

Not recommended. Use gossipsub for:
- Better scalability
- Reduced bandwidth
- Score-based peering
