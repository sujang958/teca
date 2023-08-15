<script lang="ts">
  import { invoke } from "@tauri-apps/api"
  import { listen, type UnlistenFn } from "@tauri-apps/api/event"
  import { appWindow } from "@tauri-apps/api/window"
  import { onDestroy, onMount } from "svelte"

  const unlistens: UnlistenFn[] = []

  const isPayload = (value: any): value is { key: string } => {
    if (typeof value !== "object") return false
    if (!("key" in value)) return false

    return true
  }

  let counts: { [key: string]: bigint } = {}
  let kps = 0n

  const unlisten = appWindow.onCloseRequested(() => {
    invoke("close_all")
  })

  onDestroy(() => {
    unlisten.then((f) => f())
  })

  listen("keyDown", (event) => {
    if (!isPayload(event.payload)) return

    const key = String(event.payload.key).toUpperCase()
    const keyElement = keyElements[key]

    if (!keyElement) return

    keyElement.style.backgroundColor = settings.pressedBgColor
    keyElement.style.color = settings.pressedTextColor
  }).then((v) => unlistens.push(v))

  listen("keyUp", (event) => {
    if (!isPayload(event.payload)) return

    kps += 1n
    setTimeout(() => (kps -= 1n), 1000)

    const key = String(event.payload.key).toUpperCase()
    const keyElement = keyElements[key]

    if (!keyElement) return

    keyElement.style.backgroundColor = settings.bgColor
    keyElement.style.color = settings.textColor

    counts[key] ??= 0n
    counts[key] += 1n

    counts = { ...counts }

    localStorage.setItem(
      "counts",
      JSON.stringify(
        Object.fromEntries(
          Object.entries(counts).map(([key, value]) => [key, String(value)])
        )
      )
    )
  }).then((v) => unlistens.push(v))

  const keyElements: { [key: string]: HTMLDivElement } = {}

  type Settings = {
    bgColor: string
    borderWidth: number
    borderColor: string
    textColor: string
    pressedTextColor: string
    pressedBgColor: string
  }

  let keys: string[] = []
  let settings: Settings = {
    bgColor: "#000000aa",
    borderColor: "#fff",
    borderWidth: 1,
    textColor: "#fff",
    pressedTextColor: "#000",
    pressedBgColor: "#fff",
  }

  const onStorageChange = (event: StorageEvent) => {
    if (event.key == "keys") keys = JSON.parse(localStorage.keys)
    if (event.key == "settings") settings = JSON.parse(localStorage.settings)
  }

  onMount(() => {
    window.addEventListener("unload", () => {
      alert("!")
    })

    window.addEventListener("storage", onStorageChange)

    localStorage.settings ??= JSON.stringify(settings)
    settings = JSON.parse(localStorage.settings)

    localStorage.keys ??= JSON.stringify(["D", "F", "J", "K"])
    keys = JSON.parse(localStorage.keys)

    localStorage.counts ??= "{}"

    counts = Object.fromEntries(
      Object.entries(JSON.parse(localStorage.counts))
        .filter(
          (value): value is [string, string] => typeof value[1] === "string"
        )
        .map(([key, value]) => [key, BigInt(value)])
    )
  })

  onDestroy(() => {
    window.removeEventListener("storage", onStorageChange)
    // clearInterval(countsInterval)
    unlistens.forEach((v) => v())
  })
</script>

<div class="flex flex-row items-center justify-evenly w-full gap-x-1.5">
  {#each keys as key}
    <div
      id={`KEY_${key.toUpperCase()}`}
      class="rounded-lg text-center flex-1 py-0.5"
      bind:this={keyElements[key.toUpperCase()]}
      style="background-color: {settings.bgColor}; border-color: {settings.borderColor}; border-width: {settings.borderWidth}px"
    >
      <p class="text-4xl">{key.toUpperCase()}</p>
      <p class="text-lg -mt-1">
        {counts[key.toUpperCase()] ?? 0n}
      </p>
    </div>
  {/each}
</div>

<div class="flex flex-row items-center justify-evenly w-full gap-x-1.5">
  <div
    class="not-pressed rounded-lg text-center flex-1 py-0.5 border"
    style="background-color: {settings.bgColor}; border-color: {settings.borderColor}; border-width: {settings.borderWidth}px"
  >
    <p class="text-xl">TOTAL</p>
    <p class="text-lg -mt-2">
      {Object.entries(counts)
        .filter(([key, _]) => keys.includes(key))
        .map(([_, value]) => value)
        .reduce((a, b) => a + b, 0n)}
    </p>
  </div>
  <div
    class="not-pressed rounded-lg text-center flex-1 py-0.5 border"
    style="background-color: {settings.bgColor}; border-color: {settings.borderColor}; border-width: {settings.borderWidth}px"
  >
    <p class="text-xl">KPS</p>
    <p class="text-lg -mt-2">{kps}</p>
  </div>
</div>
