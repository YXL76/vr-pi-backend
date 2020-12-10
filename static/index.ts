import "aframe";
import type { Entity } from "aframe";

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
  onClick: function () {
    const videoEl = (this.el.getAttribute("material") as {
      src: HTMLVideoElement | null;
    }).src;
    if (!videoEl) {
      return;
    }
    this.el.object3D.visible = true;
    void videoEl.play();
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

    const entity = document.getElementById("aframe-entity") as Entity;
    entity.setAttribute("material", "shader: flat; src: #remote-video");
  }
};

pc.onicecandidate = (event) => {
  if (event.candidate === null) {
    // ws.send(JSON.stringify(pc.localDescription));
  }
};

const sensor = new AbsoluteOrientationSensor();

void Promise.all([
  navigator.permissions.query({ name: "accelerometer" }),
  navigator.permissions.query({ name: "magnetometer" }),
  navigator.permissions.query({ name: "gyroscope" }),
]).then((results) => {
  if (results.every((result) => result.state === "granted")) {
    sensor.start();
  } else {
    console.log("No permissions to use AbsoluteOrientationSensor.");
  }
});

const ws = new WebSocket("ws://47.96.250.166:8080/webrtc/");

ws.onmessage = ({ data }: MessageEvent<string>) => {
  const msg = JSON.parse(data) as RTCSessionDescription;
  void pc.setRemoteDescription(new RTCSessionDescription(msg));
};

ws.onopen = () => {
  void pc.createOffer().then((d) => {
    void pc.setLocalDescription(d).then(() => {
      ws.send(JSON.stringify(d));

      sensor.onreading = () => {
        if (sensor.quaternion) {
          ws.send(JSON.stringify(sensor.quaternion));
        }
      };
    });
  });
};
