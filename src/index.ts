import "aframe";
import "webrtc-adapter";
import type { Entity } from "aframe";
import throttle from "lodash.throttle";

interface PlayOnClick {
  el?: Entity;
  init: () => void;
  play: () => void;
  pause: () => void;
  onClick: (evt: any) => void;
}

AFRAME.registerComponent<PlayOnClick>("play-on-click", {
  init: function () {
    this.onClick = this.onClick.bind(this);
  },
  play: function () {
    window.addEventListener("click", this.onClick);
  },
  pause: function () {
    window.removeEventListener("click", this.onClick);
  },
  onClick: async function () {
    const videoEl = (this.el.getAttribute("material") as {
      src: HTMLVideoElement | null;
    })?.src;
    if (!videoEl) {
      const videoEl =
        (document.getElementById("remote-video") as HTMLVideoElement) || null;

      if (!videoEl) {
        return;
      }

      await videoEl.play();
      setTimeout(() => {
        const entity = document.getElementById("aframe-entity") as Entity;
        entity.setAttribute("material", "shader: flat; src: #remote-video");
      }, 200);
    } else {
      await videoEl.play();
    }
    this.el.object3D.visible = true;
  },
});

interface HideOnPlay {
  schema: {
    type: "selector";
  };
  init: () => void;
  play: () => void;
  pause: () => void;
  onPlaying: () => void;
  onPause: () => void;
}

AFRAME.registerComponent<HideOnPlay>("hide-on-play", {
  schema: { type: "selector" },
  init: function () {
    this.onPlaying = this.onPlaying.bind(this);
    this.onPause = this.onPause.bind(this);
  },
  play: function () {
    if (this.data) {
      (this.data as EventTarget).addEventListener("playing", this.onPlaying);
      (this.data as EventTarget).addEventListener("pause", this.onPause);
    }
  },
  pause: function () {
    if (this.data) {
      (this.data as EventTarget).removeEventListener("playing", this.onPlaying);
      (this.data as EventTarget).removeEventListener("pause", this.onPause);
    }
  },
  onPlaying: function () {
    this.el.setAttribute("visible", false);
  },
  onPause: function () {
    this.el.setAttribute("visible", true);
  },
});

const pc = new RTCPeerConnection({
  iceServers: [
    {
      urls: "stun:stun.l.google.com:19302",
    },
  ],
});

pc.addTransceiver("video", { direction: "sendrecv" });

pc.ontrack = ({ track, streams }) => {
  if (track.kind === "video") {
    const el = (document.createElement(
      track.kind
    ) as unknown) as HTMLVideoElement;
    el.id = "remote-video";
    el.srcObject = streams[0];
    el.autoplay = true;
    el.controls = true;

    document.getElementById("aframe-assets")?.appendChild(el);
  }
};

/* pc.onicecandidate = (event) => {
  if (event.candidate === null) {
    webrtcConn.send(JSON.stringify(pc.localDescription));
  }
}; */

// const options = { frequency: 50 };

// const relaOrie = new RelativeOrientationSensor(options);
/* const accl = new Accelerometer(options);
const gyro = new Gyroscope(options);

let timestamp: number | null = null;
let alpha = 0;
let beta = 0;
let gamma = 0;
const bias = 0.98;
const oneMinusBias = 1 - bias;
const scale = Math.PI / 2;

const calaEulerAngles = () => {
  const dt = timestamp
    ? ((gyro as { timestamp: number }).timestamp - timestamp) / 1000
    : 0;
  timestamp = (gyro as { timestamp: number }).timestamp;

  const norm = Math.sqrt(
    (accl as { x: number }).x ** 2 +
      (accl as { y: number }).y ** 2 +
      (accl as { z: number }).z ** 2
  );

  alpha = alpha + (gyro as { z: number }).z * dt;
  beta =
    bias * (beta + (gyro as { x: number }).x * dt) +
    oneMinusBias * (((accl as { x: number }).x * scale) / norm);
  gamma =
    bias * (gamma + (gyro as { y: number }).y * dt) +
    oneMinusBias * (((accl as { y: number }).y * -scale) / norm);

  webrtcConn.send(JSON.stringify({ alpha, beta, gamma }));
};
 */
/* const halfPi = Math.PI / 2;

const calaEulerAngles = () => {
  if (relaOrie.quaternion) {
    const [x, y, z, w] = relaOrie.quaternion;

    const sinr_cosp = w * x + y * z;
    const cosr_cosp = 1 - 2 * (x ** 2 + y ** 2);
    const roll = Math.atan2(sinr_cosp, cosr_cosp);

    const sinp = 2 * (w * y - z * x);
    let pitch;
    if (sinp >= 1) {
      pitch = halfPi;
    } else if (sinp <= -1) {
      pitch = -halfPi;
    } else {
      pitch = Math.asin(sinp);
    }

    const siny_cosp = 2 * (w * z + x * y);
    const cosy_cosp = 1 - 2 * (y ** 2 + z ** 2);
    const yaw = Math.atan2(siny_cosp, cosy_cosp);
    webrtcConn.send(JSON.stringify({ roll, pitch, yaw }));
  }
}; */

void Promise.all([
  navigator.permissions.query({ name: "accelerometer" }),
  navigator.permissions.query({ name: "magnetometer" }),
  navigator.permissions.query({ name: "gyroscope" }),
]).then((results) => {
  if (results.every((result) => result.state === "granted")) {
    // relaOrie.start();
    // accl.start();
    // gyro.start();
  } else {
    console.log("No permissions to use AbsoluteOrientationSensor.");
  }
});

const sensorConn = new WebSocket("wss://www.cangcheng.top/sensor");

const sensorHandler = throttle(({ alpha, gamma }: DeviceOrientationEvent) => {
  sensorConn.send(JSON.stringify({ alpha, gamma }));
}, 16);

sensorConn.onopen = () => {
  window.addEventListener("deviceorientation", sensorHandler);
};

const webrtcConn = new WebSocket("wss://www.cangcheng.top/webrtc");

webrtcConn.onmessage = ({ data }: MessageEvent<string>) => {
  const msg = JSON.parse(data) as RTCSessionDescription;
  void pc.setRemoteDescription(new RTCSessionDescription(msg));
};

webrtcConn.onopen = () => {
  void pc
    .createOffer()
    .then((d) => pc.setLocalDescription(d))
    .then(() => webrtcConn.send(JSON.stringify(pc.localDescription)));
};
