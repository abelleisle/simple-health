import './style.css'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div class="max-w-4xl mx-auto p-8">
    <h1 class="text-3xl font-bold text-gray-800 mb-8">Simple Health</h1>
    <p class="text-lg text-gray-600 mb-8">Welcome to your health tracking dashboard!</p>
    <div class="bg-gray-50 p-6 rounded-lg">
      <h2 class="text-xl font-semibold text-gray-700 mt-0 mb-4">Getting Started</h2>
      <p class="text-gray-600 mb-0">Your web frontend is ready to go. Start building your health tracking features here!</p>
    </div>
  </div>
`
