<script lang="ts">
  import { listen, type UnlistenFn } from "@tauri-apps/api/event"
  import { onDestroy, onMount } from "svelte"

  const unlistens: UnlistenFn[] = []

  const isPayload = (value: any): value is { key: string } => {
    if (typeof value !== "object") return false
    if (!("key" in value)) return false

    return true
  }

  let counts: { [key: string]: bigint } = {}
  let tempCounts = 0n
  let kps = 0n

  const countsInterval = setInterval(() => {
    kps = tempCounts
    tempCounts = 0n
  }, 1000)

  listen("keyDown", (event) => {
    if (!isPayload(event.payload)) return

    const key = String(event.payload.key).toUpperCase()

    keyElements[key].classList.remove("not-pressed")
    keyElements[key].classList.add("pressed")
  }).then((v) => unlistens.push(v))

  listen("keyUp", (event) => {
    if (!isPayload(event.payload)) return

    tempCounts += 1n

    const key = String(event.payload.key).toUpperCase()

    keyElements[key]?.classList.add("not-pressed")
    keyElements[key]?.classList.remove("pressed")

    counts[key] ??= 0n
    counts[key] += 1n

    counts = { ...counts }

    localStorage.setItem(
      "counts",
      JSON.stringify(
        Object.fromEntries(
          Object.entries(counts).map(([key, value]) => [key, String(value)]),
        ),
      ),
    )
  }).then((v) => unlistens.push(v))

  let targetKeys: string[] = []

  const keyElements: { [key: string]: HTMLDivElement } = {}

  onMount(() => {
    localStorage.keys ??= '["D", "F", "J", "K"]'
    targetKeys = JSON.parse(localStorage.keys)

    localStorage.counts ??= "{}"

    counts = Object.fromEntries(
      Object.entries(JSON.parse(localStorage.counts))
        .filter(
          (value): value is [string, string] => typeof value[1] === "string",
        )
        .map(([key, value]) => [key, BigInt(value)]),
    )
  })

  onDestroy(() => {
    clearInterval(countsInterval)
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
      <p class="text-lg -mt-1">
        {counts[targetKey.toUpperCase()] ?? 0n}
      </p>
    </div>
  {/each}
</div>

<div class="flex flex-row items-center justify-evenly w-full gap-x-1.5">
  <div
    class="not-pressed rounded-lg text-center flex-1 py-0.5 border border-neutral-400"
  >
    <p class="text-xl">TOTAL</p>
    <p class="text-lg -mt-2">
      {Object.entries(counts)
        .map(([_, value]) => value)
        .reduce((a, b) => a + b, 0n)}
    </p>
  </div>
  <div
    class="not-pressed rounded-lg text-center flex-1 py-0.5 border border-neutral-400"
  >
    <p class="text-xl">KPS</p>
    <p class="text-lg -mt-2">{kps}</p>
  </div>
</div>
