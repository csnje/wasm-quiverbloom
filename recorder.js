export class CanvasRecorder {
    constructor(canvas, filename) {
        this._canvas = canvas;
        this._filename = filename;

        this._isRecording = false;
        this._chunks = [];

        this._generator = new MediaStreamTrackGenerator({ kind: 'video' });
        this._writer = this._generator.writable.getWriter();

        this._stream = new MediaStream([this._generator]);
        this._recorder = new MediaRecorder(this._stream, { mimeType: 'video/webm' });

        this._setupRecorderEvents();
    }

    _setupRecorderEvents() {
        this._recorder.ondataavailable = (e) => {
            this._chunks.push(e.data);
        };

        this._recorder.onstop = () => {
            this._saveVideo();
            this._isRecording = false;
            this._chunks = [];
        };
    }

    start() {
        if (!this._isRecording) {
            console.debug('starting recording');
            this._isRecording = true;
            this._recorder.start();
        }
    }

    async captureFrame() {
        if (this._isRecording) {
            const videoFrame = new VideoFrame(this._canvas, { timestamp: performance.now() });
            await this._writer.write(videoFrame);
            videoFrame.close();
        }
    }

    stop() {
        if (this._isRecording) {
            console.debug('stopping recording');
            this._writer.close();
            this._recorder.stop();
        }
    }

    _saveVideo() {
        console.debug('saving recording');
        const blob = new Blob(this._chunks, { 'type': 'video/webm' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = this._filename;
        a.click();
        a.remove();
        URL.revokeObjectURL(url);
    }
}

// consider post-processing the saved recording to add metadata, for example:
// `ffmpeg -i saved_recording.webm -c:v copy processed_recording.webm`
