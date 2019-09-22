const PARTICLE_SIZE = 20;

var iters = 0;
var isInitialized = false;

var renderer, scene, camera;
var particles;

function buildGeometry(bigbangPositions) {
  var positions = new Float32Array((bigbangPositions.length / 4) * 3);
  var colors = new Float32Array((bigbangPositions.length / 4) * 3);
  var sizes = new Float32Array(bigbangPositions.length / 4);

  var color = new THREE.Color();
  for (let i = 0, l = bigbangPositions.length / 4; i < l; i += 4) {
    positions[i * 3 + 0] = bigbangPositions[i * 4 + 0];
    positions[i * 3 + 1] = bigbangPositions[i * 4 + 1];
    positions[i * 3 + 2] = bigbangPositions[i * 4 + 2];

    color.setHSL(0.01 + 0.1 * (i / l), 1.0, 0.5);
    color.toArray(colors, i * 3);
    sizes[i] = PARTICLE_SIZE * 0.5;
  }

  var geometry = new THREE.BufferGeometry();
  geometry.addAttribute('position', new THREE.BufferAttribute(positions, 3));
  geometry.addAttribute('customColor', new THREE.BufferAttribute(colors, 3));
  geometry.addAttribute('size', new THREE.BufferAttribute(sizes, 1));
}

function init(bigbangPositions) {
  var container = document.getElementById('container');
  scene = new THREE.Scene();
  camera = new THREE.PerspectiveCamera(45, window.innerWidth / window.innerHeight, 1, 10000);
  camera.position.z = 250;

  const geometry = buildGeometry(bigbangPositions);

  var material = new THREE.ShaderMaterial({
    uniforms: {
      color: { value: new THREE.Color(0xffffff) },
      pointTexture: { value: new THREE.TextureLoader().load('./disc.png') },
    },
    vertexShader: document.getElementById('vertexshader').textContent,
    fragmentShader: document.getElementById('fragmentshader').textContent,
    alphaTest: 0.9,
  });

  particles = new THREE.Points(geometry, material);
  scene.add(particles);

  renderer = new THREE.WebGLRenderer();
  renderer.setPixelRatio(window.devicePixelRatio);
  renderer.setSize(window.innerWidth, window.innerHeight);
  container.appendChild(renderer.domElement);

  raycaster = new THREE.Raycaster();

  window.addEventListener('resize', onWindowResize, false);
}

function onWindowResize() {
  camera.aspect = window.innerWidth / window.innerHeight;
  camera.updateProjectionMatrix();
  renderer.setSize(window.innerWidth, window.innerHeight);
}

function render() {
  particles.rotation.x += 0.0005;
  particles.rotation.y += 0.001;

  var geometry = particles.geometry;
  var attributes = geometry.attributes;
  // console.log(attributes);

  renderer.render(scene, camera);
}

const handleMessage = async msg => {
  if (iters === 0) {
    // console.log(msg.data);
    iters += 1;
    return;
  }
  const data = new Float64Array(await msg.data.arrayBuffer());

  for (let i = 0; i < data.length; i += 4) {
    const x = data[i * 4 + 0];
    const y = data[i * 4 + 1];
    const z = data[i * 4 + 2];
    const radius = data[i * 4 + 3];
    if (iters === 1) {
      console.log(`x: ${x}, y: ${y}, z: ${z}, radius: ${radius}`);
    }
  }

  iters += 1;

  if (!isInitialized) {
    isInitialized = true;
    init(data);
  }

  render();
};

window.onload = () => {
  const container = document.getElementById('container');
  if (!container) {
    throw new Error('Failed to find #container element');
  }

  const ws = new WebSocket('ws://localhost:3355', ['rust-websocket']);

  ws.onmessage = handleMessage;
};
