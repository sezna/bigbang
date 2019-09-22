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
    positions[i * 3 + 0] = bigbangPositions[i * 4 + 0] / 1;
    positions[i * 3 + 1] = bigbangPositions[i * 4 + 1] / 1;
    positions[i * 3 + 2] = bigbangPositions[i * 4 + 2] / 1;

    color.setHSL(0.01 + 0.1 * (i / l), 1.0, 0.5);
    color.toArray(colors, i * 3);
    sizes[i] = bigbangPositions[i * 4 + 3] * PARTICLE_SIZE;
  }

  var geometry = new THREE.BufferGeometry();
  geometry.addAttribute('position', new THREE.BufferAttribute(positions, 3));
  geometry.addAttribute('customColor', new THREE.BufferAttribute(colors, 3));
  geometry.addAttribute('size', new THREE.BufferAttribute(sizes, 1));

  return geometry;
}

function init(bigbangPositions) {
  var container = document.getElementById('container');
  scene = new THREE.Scene();
  camera = new THREE.PerspectiveCamera(45, window.innerWidth / window.innerHeight, 1, 10000);
  camera.position.z = 250;

  const geometry = buildGeometry(bigbangPositions);
  console.log(geometry);

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

function render(bigbangPositions) {
  particles.rotation.x += 0.0005;
  particles.rotation.y += 0.001;

  var geometry = particles.geometry;
  var attributes = geometry.attributes;

  const newGeometry = buildGeometry(bigbangPositions);
  attributes.position.array = newGeometry.attributes.position.array;
  attributes.position.needsUpdate = true;
  attributes.size.array = newGeometry.attributes.size.array;
  attributes.size.needsUpdate = true;

  renderer.render(scene, camera);
}

const handleMessage = async msg => {
  if (iters === 0) {
    iters += 1;
    // ignore welcome message
    console.log(msg.data);
    return;
  }

  const data = new Float64Array(await msg.data.arrayBuffer());

  iters += 1;

  if (!isInitialized) {
    isInitialized = true;
    init(data);
  } else {
    render(data);
  }
};

window.onload = () => {
  const container = document.getElementById('container');
  if (!container) {
    throw new Error('Failed to find #container element');
  }

  const ws = new WebSocket('ws://localhost:3355', ['rust-websocket']);

  ws.onmessage = handleMessage;
};
