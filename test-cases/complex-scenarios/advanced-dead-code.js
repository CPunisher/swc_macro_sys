// Advanced Dead Code Elimination - Complex Dependency Chains
// Testing DCE with function dependency graphs where entire chains should be eliminated

// Heavy computation utilities (no macros on definitions)
function performMatrixMultiplication(size) {
  console.log(`Performing ${size}x${size} matrix multiplication - very expensive!`);
  const matrix = Array(size).fill().map(() => Array(size).fill(1));
  // Simulate expensive computation
  for (let i = 0; i < size; i++) {
    for (let j = 0; j < size; j++) {
      matrix[i][j] = Math.random() * 100;
    }
  }
  return { matrix, computationTime: `${size * size}ms` };
}

function optimizeAlgorithm(data) {
  console.log('Running advanced algorithm optimization');
  return data.map(item => ({
    ...item,
    optimized: true,
    score: Math.random() * 100
  }));
}

function generateHeavyReport(computedData) {
  console.log('Generating comprehensive report - expensive I/O operations');
  return {
    summary: `Processed ${computedData.length} items`,
    details: computedData,
    metadata: {
      generated: new Date().toISOString(),
      complexity: 'high',
      processingTime: '3.2s'
    }
  };
}

// ML/AI simulation functions
function trainNeuralNetwork(dataset) {
  console.log('Training neural network - extremely expensive operation');
  return {
    model: 'trained-model-v1',
    accuracy: 0.95,
    trainingTime: '45 minutes',
    parameters: dataset.length * 1000
  };
}

function runInference(model, input) {
  console.log('Running ML inference');
  return {
    prediction: Math.random() > 0.5 ? 'positive' : 'negative',
    confidence: Math.random(),
    model: model.model
  };
}

// Graphics and rendering utilities
function initializeGraphicsEngine() {
  console.log('Initializing advanced graphics engine');
  return {
    renderer: '3D-advanced',
    shaders: ['vertex', 'fragment', 'geometry'],
    initialized: true
  };
}

function renderComplexScene(engine, sceneData) {
  console.log('Rendering complex 3D scene with advanced lighting');
  return {
    engine: engine.renderer,
    objects: sceneData.objects,
    renderTime: '250ms',
    triangles: sceneData.objects * 10000
  };
}

function applyPostProcessing(renderedScene) {
  console.log('Applying post-processing effects');
  return {
    ...renderedScene,
    effects: ['bloom', 'ambient-occlusion', 'depth-of-field'],
    postProcessTime: '150ms'
  };
}

// Analytics and telemetry functions
function collectTelemetryData() {
  console.log('Collecting detailed telemetry data');
  return {
    metrics: ['cpu', 'memory', 'gpu', 'network'],
    samples: 1000,
    collectionTime: Date.now()
  };
}

function processAnalytics(telemetryData) {
  console.log('Processing analytics with ML algorithms');
  return {
    insights: ['performance bottleneck detected', 'memory usage spike'],
    recommendations: ['optimize shader code', 'reduce texture size'],
    processingTime: '500ms'
  };
}

// Main application functions that conditionally call the above

export function runDataProcessingPipeline() {
  console.log('Starting data processing pipeline');
  
  const inputData = [
    { id: 1, value: 100 },
    { id: 2, value: 200 },
    { id: 3, value: 300 }
  ];
  
  // Heavy computation chain - should be eliminated if condition is false
  /* @common:if [condition="features.enableHeavyComputation"] */
  const matrixResult = performMatrixMultiplication(100);
  const optimizedData = optimizeAlgorithm(inputData);
  const report = generateHeavyReport(optimizedData);
  console.log('Heavy computation completed:', report.summary);
  /* @common:endif */
  
  return {
    status: 'pipeline completed',
    inputCount: inputData.length
  };
}

export function runMLWorkflow() {
  console.log('ML workflow starting');
  
  const trainingData = Array(1000).fill().map((_, i) => ({ 
    feature: Math.random(), 
    label: i % 2 
  }));
  
  // ML pipeline - should be eliminated if AI features disabled
  /* @common:if [condition="features.enableAI"] */
  const model = trainNeuralNetwork(trainingData);
  const predictions = trainingData.slice(0, 10).map(data => 
    runInference(model, data)
  );
  console.log(`ML workflow completed with ${predictions.length} predictions`);
  /* @common:endif */
  
  return {
    status: 'ml workflow completed',
    dataSize: trainingData.length
  };
}

export function runGraphicsApplication() {
  console.log('Graphics application starting');
  
  const sceneData = {
    objects: 500,
    lights: 10,
    cameras: 2
  };
  
  // Graphics pipeline - should be eliminated if graphics disabled
  /* @common:if [condition="features.enableAdvancedGraphics"] */
  const engine = initializeGraphicsEngine();
  const renderedScene = renderComplexScene(engine, sceneData);
  const finalScene = applyPostProcessing(renderedScene);
  console.log(`Graphics rendered: ${finalScene.triangles} triangles`);
  /* @common:endif */
  
  return {
    status: 'graphics application completed',
    sceneComplexity: sceneData.objects
  };
}

export function runAnalyticsDashboard() {
  console.log('Analytics dashboard starting');
  
  // Analytics pipeline - should be eliminated if analytics disabled
  /* @common:if [condition="features.enableAnalytics"] */
  const telemetry = collectTelemetryData();
  const insights = processAnalytics(telemetry);
  console.log(`Analytics completed with ${insights.insights.length} insights`);
  /* @common:endif */
  
  return {
    status: 'analytics dashboard completed'
  };
}

// Test function with isolated calls
export function testIsolatedHeavyComputation() {
  // This should eliminate performMatrixMultiplication if condition is false
  /* @common:if [condition="features.enableExperimentalMath"] */
  const result = performMatrixMultiplication(50);
  console.log('Experimental math result:', result.computationTime);
  /* @common:endif */
  
  return 'Isolated test completed';
}

// Test function with dependency chains
export function testDependencyChain() {
  const data = [{ id: 1, value: 42 }];
  
  // This chain should be eliminated entirely if condition is false
  /* @common:if [condition="features.enableFullPipeline"] */
  const optimized = optimizeAlgorithm(data);
  const report = generateHeavyReport(optimized);
  console.log('Full pipeline completed:', report.metadata.processingTime);
  /* @common:endif */
  
  return 'Dependency chain test completed';
}

// Test multiple conditional calls to same function
export function testMultipleConditionalUsage() {
  const testData = [{ test: true }];
  
  /* @common:if [condition="features.enablePathA"] */
  const resultA = optimizeAlgorithm(testData);
  console.log('Path A optimization:', resultA.length);
  /* @common:endif */
  
  /* @common:if [condition="features.enablePathB"] */
  const resultB = optimizeAlgorithm(testData);
  console.log('Path B optimization:', resultB.length);
  /* @common:endif */
  
  return 'Multiple usage test completed';
}

// Always present baseline
export function baselineFunction() {
  return {
    message: 'Baseline functionality always available',
    timestamp: Date.now()
  };
} 