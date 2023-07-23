<script lang="ts">
  import { listen, type UnlistenFn } from "@tauri-apps/api/event"
  import { onDestroy, onMount } from "svelte"

  const unlistens: UnlistenFn[] = []

  listen("keyDown", (event) => {
    if (typeof event.payload !== "object") return
    if (!("key" in event.payload)) return

    const key = String(event.payload.key).toUpperCase()

    keyElements[key].classList.remove("not-pressed")
    keyElements[key].classList.add("pressed")
  }).then((v) => unlistens.push(v))

  listen("keyUp", (event) => {
    if (typeof event.payload !== "object") return
    if (!("key" in event.payload)) return

    const key = String(event.payload.key).toUpperCase()

    keyElements[key].classList.add("not-pressed")
    keyElements[key].classList.remove("pressed")
  }).then((v) => unlistens.push(v))

  let targetKeys: string[] = []

  const keyElements: { [key: string]: HTMLDivElement } = {}

  onMount(() => {
    localStorage.keys ??= '["D", "F", "J", "K"]'
    targetKeys = JSON.parse(localStorage.keys)
  })

  onDestroy(() => {
    unlistens.forEach((v) => v())
  })
</script>

<div class="flex flex-row items-center justify-evenly w-full gap-x-1.5">
  {#each targetKeys as targetKey}
    <div
      id={`KEY_${targetKey.toUpperCase()}`}
      class="not-pressed rounded-lg text-center flex-1 py-0.5 border border-neutral-400"
      bind:this={keyElements[targetKey.toUpperCase()]}
    >
      <p class="text-4xl">{targetKey.toUpperCase()}</p>
      <p class="text-lg -mt-1">399</p>
    </div>
  {/each}
</div>

<div class="flex flex-row items-center justify-evenly w-full gap-x-1">
  <div
    class="not-pressed rounded-lg text-center flex-1 py-0.5 border border-neutral-400"
  >
    <p class="text-xl">TOTAL</p>
    <p class="text-lg -mt-2">69 M</p>
  </div>
  <div
    class="not-pressed rounded-lg text-center flex-1 py-0.5 border border-neutral-400"
  >
    <p class="text-xl">KPS</p>
    <p class="text-lg -mt-2">74</p>
  </div>
</div>
