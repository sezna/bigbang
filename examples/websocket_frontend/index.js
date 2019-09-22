const PARTICLE_SIZE = 20;
// How much to scale the particle size by when zooming.
const ZOOM_TO_SIZE_SCALE = 2;
// How many coordinates are moved when moving camera in response to user input
const TRANSLATION_STEP_SIZE = 80;

var iters = 0;
var isInitialized = false;

var renderer, scene, camera;
var particles;

var zoomFactor = 0.2;
var rotationX = 0;
var rotationY = 0;
var rotationZ = 0;

var particlesOffsets = { x: 0, y: 0, z: 0 };

const initControlPanel = () => {
  const inputs = [
    { type: 'range', label: 'zoom', min: 0.01, max: 5, step: 0.001, initial: zoomFactor },
    { type: 'range', label: 'rotation_x', min: 0, max: Math.PI * 2, step: 0.1, initial: 0 },
    { type: 'range', label: 'rotation_y', min: 0, max: Math.PI * 2, step: 0.1, initial: 0 },
    { type: 'range', label: 'rotation_z', min: 0, max: Math.PI * 2, step: 0.1, initial: 0 },
  ];

  var panelNode = document.body.appendChild(document.createElement('div'));
  panelNode.setAttribute('id', 'viz-controls');
  const panel = control(inputs, {
    theme: 'dark',
    title: 'visualization controls',
    root: panelNode,
  });

  panel.on('input', ({ zoom, rotation_x, rotation_y, rotation_z }) => {
    zoomFactor = zoom;
    rotationX = rotation_x;
    rotationY = rotation_y;
    rotationZ = rotation_z;
  });
};

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
    sizes[i] = bigbangPositions[i * 4 + 3] * PARTICLE_SIZE * (zoomFactor * ZOOM_TO_SIZE_SCALE);
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
  camera.position.x = 0;
  camera.position.y = 0;
  camera.position.z = -1000;
  camera.lookAt(new THREE.Vector3(0, 0, 0));

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
  console.log(particles);

  renderer = new THREE.WebGLRenderer();
  renderer.setPixelRatio(window.devicePixelRatio);
  renderer.setSize(window.innerWidth, window.innerHeight);
  container.appendChild(renderer.domElement);

  raycaster = new THREE.Raycaster();

  window.addEventListener('resize', onWindowResize, false);

  initControlPanel();

  document.addEventListener('keypress', evt => {
    switch (evt.key) {
      case 'd':
      case 'ArrowRight':
        particlesOffsets.x += TRANSLATION_STEP_SIZE;
        break;
      case 'a':
      case 'ArrowLeft':
        particlesOffsets.x -= TRANSLATION_STEP_SIZE;
        break;
      case 'w':
      case 'ArrowUp':
        particlesOffsets.y += TRANSLATION_STEP_SIZE;
        break;
      case 's':
      case 'ArrowDown':
        particlesOffsets.y -= TRANSLATION_STEP_SIZE;
        break;
      case '=':
        particlesOffsets.z -= TRANSLATION_STEP_SIZE;
        break;
      case '-':
        particlesOffsets.z += TRANSLATION_STEP_SIZE;
        break;

      default:
    }
    console.log(particlesOffsets, evt.key);
  });
}

function onWindowResize() {
  camera.aspect = window.innerWidth / window.innerHeight;
  console.log(camera);
  camera.updateProjectionMatrix();
  renderer.setSize(window.innerWidth, window.innerHeight);
}

function render(bigbangPositions) {
  camera.rotation.x = rotationX;
  camera.rotation.y = rotationY;
  camera.rotation.z = rotationZ;
  camera.position.x = particlesOffsets.x;
  camera.position.y = particlesOffsets.y;
  camera.position.z = particlesOffsets.z;
  camera.zoom = zoomFactor;
  camera.updateProjectionMatrix();

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
