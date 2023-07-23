<script lang="ts">
  import { listen, type UnlistenFn } from "@tauri-apps/api/event"
  import { onDestroy } from "svelte"

  const unlistens: UnlistenFn[] = []

  listen("keyDown", (event) => {
    console.log("Down", event.payload)
  }).then((v) => unlistens.push(v))

  listen("keyUp", (event) => {
    console.log("Up", event.payload)
  }).then((v) => unlistens.push(v))

  onDestroy(() => {
    unlistens.forEach((v) => v())
  })
</script>

<h1>hi</h1>
