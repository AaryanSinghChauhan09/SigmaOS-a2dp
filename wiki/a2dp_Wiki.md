# SigmaOS-a2dp Component Wiki

## Overview
SigmaOS-a2dp is a SigmaOS component providing A2DP (Advanced Audio Distribution Profile) Bluetooth audio streaming support for the SigmaOS desktop environment. It demonstrates Bluetooth audio integration and wireless audio streaming within the SigmaOS ecosystem.

## Principles
- Single Responsibility
- Bluetooth Audio Abstraction
- Fail-Fast on Audio Streaming Errors
- Deterministic Streaming Initialization
- Clear Abstraction Over A2DP API
- Composition over Inheritance for audio codecs

## Architecture
SigmaOS-a2dp follows a Bluetooth audio streaming pattern:
- **Codec Management**: Handles selection and management of audio codecs (aptX, SBC, AAC)
- **Stream Configuration**: Manages audio channels, sample rates, and bitrates
- **Device Connection**: Handles Bluetooth device pairing and connection lifecycle
- **Error Handling**: Fail-fast on streaming failures or unsupported codecs

## API
The component provides the following public APIs:
- `a2dp::codec::list()` - List supported audio codecs
- `a2dp::codec::capabilities(codec_id)` - Get codec capabilities
- `a2dp::stream::start(stream_id)` - Start audio streaming
- `a2dp::stream::stop(stream_id)` - Stop audio streaming
- `a2dp::device::connect(device_id)` - Connect to Bluetooth device
- Error types for unsupported codecs or streaming failures

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific audio codec configuration
2. **Act**: Execute the code under test (codec selection, stream setup)
3. **Assert**: Verify the expected audio streaming results

Example:
```rust
#[test]
fn test_a2dp_codec_selection_aaa() {
    // Arrange
    let preferred_codecs = vec!["aptX".to_string(), "SBC".to_string()];
    
    // Act
    let selected = a2dp::codec::select(preferred_codecs);
    
    // Assert
    assert!(selected.is_some());
    assert!(selected.unwrap().contains("SBC"));  // SBC is mandatory fallback
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- A2DP streaming is best-effort; gracefully degrades on unsupported codecs
- Comprehensive error types for all streaming failure scenarios
- Unit tests provided for codec selection and stream configuration
- Integration tests for full A2DP flow with test device mocks

## TODO
- Add support for aptX HD and LDAC codecs
- Expand device connection management
- Add audio format configuration APIs
- Create integration tests with actual Bluetooth hardware
- Implement codec priority and fallback logic