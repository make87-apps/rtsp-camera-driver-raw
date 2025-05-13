# RTSP Camera Driver: Raw Stream Publisher

This application connects to an RTSP camera stream, decodes H.264/H.265 video into raw RGB or YUV frames using FFmpeg, and
publishes each frame as an `ImageRawAny` message on the make87 platform.

## 📦 Features

- Connects to RTSP video streams over TCP
- Decodes video frames using FFmpeg
- Converts frames to uncompressed (currently:) RGB888 or YUV420
- Publishes frames to the `IMAGE_RAW` topic using the `ImageRawAny` message type
- Filters by stream index to support cameras with multiple outputs
- Automatically sets the `entity_path` as `/camera/<ip>/<uri_suffix>`

## 🔧 Configuration

This app uses the following configuration values:

| Name              | Required | Default | Description                                       |
|-------------------|----------|---------|---------------------------------------------------|
| IMAGE_FORMAT      | Yes      | -       | Image format to decode (e.g., `RGB888`, `YUV420`) |
| CAMERA_USERNAME   | Yes      | –       | Username for RTSP login                           |
| CAMERA_PASSWORD   | Yes      | –       | Password for RTSP login                           |
| CAMERA_IP         | Yes      | –       | IP address of the RTSP camera                     |
| CAMERA_PORT       | No       | 554     | RTSP port number                                  |
| CAMERA_URI_SUFFIX | No       | (empty) | Optional URI suffix (e.g., `stream1`, `live`)     |
| STREAM_INDEX      | No       | 0       | Stream index for multi-stream cameras             |

You can add multiple cameras by using comma-separated values for every field.
The`CAMERA_PASSWORD` values are automatically URL-encoded.

## 📤 Output

Each decoded frame is published as an `ImageRawAny` message on the `IMAGE_RAW` topic. The `Header` includes the current
wallclock timestamp and the computed `entity_path`.

## 💡 Notes

- Only the latest decoded frame is kept in memory after decoding. Older frames are dropped to reduce latency.
- This app uses a `tokio::sync::watch` channel for efficient zero-queue, drop-old behavior.
- The `entity_path` is derived automatically from the RTSP URL: `/camera/<ip>/<uri_suffix>`.

---

© make87, 2025
