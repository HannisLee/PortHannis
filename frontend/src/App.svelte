<script lang="ts">
  import './app.css'
  import { onMount } from 'svelte'
  import { Greet } from '../wailsjs/go/main/App'

  let resultText = ''
  let greetName = ''

  async function doGreet() {
    resultText = await Greet(greetName)
  }

  onMount(() => {
    resultText = 'Welcome to PortHannis!'
  })
</script>

<main>
  <div id="result" class="result">{resultText}</div>
  <div id="input" class="input-box">
    <input id="name" type="text" autocomplete="off" bind:value={greetName} class="input" />
    <button class="btn" on:click={doGreet}>Greet</button>
  </div>
</main>

<style>
  :root {
    font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
    line-height: 1.5;
    font-weight: 400;
    color: #213547;
    background-color: #ffffff;
  }

  main {
    text-align: center;
    padding: 2rem;
    height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  .result {
    margin-bottom: 2rem;
    font-size: 1.5rem;
  }

  .input-box {
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .input {
    padding: 0.5rem 1rem;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
    font-size: 1rem;
    width: 200px;
  }

  .btn {
    padding: 0.5rem 1.5rem;
    background-color: #646cff;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    cursor: pointer;
  }

  .btn:hover {
    background-color: #535bf2;
  }
</style>
