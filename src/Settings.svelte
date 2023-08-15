<script lang="ts">
  import { invoke } from "@tauri-apps/api"
  import { appWindow } from "@tauri-apps/api/window"
  import { onDestroy } from "svelte"

  const unlisten = appWindow.onCloseRequested(() => {
    invoke("close_all")
  })

  onDestroy(() => {
    unlisten.then((f) => f())
  })
</script>

<div class="flex flex-col gap-y-4">
  <p class="text-3xl font-semibold">Settings</p>
  <div class="py-1" />
  <label class="text-sm flex flex-col gap-y-2">
    Keys
    <input
      type="text"
      class="rounded-lg bg-neutral-950 border border-neutral-700 px-1.5 outline-none w-min text-base py-1"
      value={JSON.parse(localStorage.keys).join(",")}
      on:input={(event) => {
        if (!(event.target instanceof HTMLInputElement)) return

        localStorage.keys = JSON.stringify(
          event.target.value.replaceAll(" ", "").split(",")
        )
      }}
    />
  </label>
  <label class="text-sm flex flex-col gap-y-2">
    Background Color
    <input
      type="text"
      class="rounded-lg bg-neutral-950 border border-neutral-700 px-1.5 outline-none w-min text-base py-1"
      value={JSON.parse(localStorage.settings).bgColor}
      on:input={(event) => {
        if (!(event.target instanceof HTMLInputElement)) return

        localStorage.settings = JSON.stringify({
          ...JSON.parse(localStorage.settings),
          bgColor: event.target.value,
        })
      }}
    />
  </label>
  <label class="text-sm flex flex-col gap-y-2">
    Text Color
    <input
      type="text"
      class="rounded-lg bg-neutral-950 border border-neutral-700 px-1.5 outline-none w-min text-base py-1"
      value={JSON.parse(localStorage.settings).textColor}
      on:input={(event) => {
        if (!(event.target instanceof HTMLInputElement)) return

        localStorage.settings = JSON.stringify({
          ...JSON.parse(localStorage.settings),
          textColor: event.target.value,
        })
      }}
    />
  </label>
  <label class="text-sm flex flex-col gap-y-2">
    Background Color when Hover
    <input
      type="text"
      class="rounded-lg bg-neutral-950 border border-neutral-700 px-1.5 outline-none w-min text-base py-1"
      value={JSON.parse(localStorage.settings).pressedBgColor}
      on:input={(event) => {
        if (!(event.target instanceof HTMLInputElement)) return

        localStorage.settings = JSON.stringify({
          ...JSON.parse(localStorage.settings),
          pressedBgColor: event.target.value,
        })
      }}
    />
  </label>
  <label class="text-sm flex flex-col gap-y-2">
    Text Color when pressed
    <input
      type="text"
      class="rounded-lg bg-neutral-950 border border-neutral-700 px-1.5 outline-none w-min text-base py-1"
      value={JSON.parse(localStorage.settings).pressedTextColor}
      on:input={(event) => {
        if (!(event.target instanceof HTMLInputElement)) return

        localStorage.settings = JSON.stringify({
          ...JSON.parse(localStorage.settings),
          pressedTextColor: event.target.value,
        })
      }}
    />
  </label>
  <label class="text-sm flex flex-col gap-y-2">
    Border Width
    <input
      type="number"
      class="rounded-lg bg-neutral-950 border border-neutral-700 px-1.5 outline-none w-min text-base py-1"
      value={JSON.parse(localStorage.settings).borderWidth}
      on:input={(event) => {
        if (!(event.target instanceof HTMLInputElement)) return

        localStorage.settings = JSON.stringify({
          ...JSON.parse(localStorage.settings),
          borderWidth: event.target.valueAsNumber,
        })
      }}
    />
  </label>
  <label class="text-sm flex flex-col gap-y-2">
    Border Color
    <input
      type="text"
      class="rounded-lg bg-neutral-950 border border-neutral-700 px-1.5 outline-none w-min text-base py-1"
      value={JSON.parse(localStorage.settings).borderColor}
      on:input={(event) => {
        if (!(event.target instanceof HTMLInputElement)) return

        localStorage.settings = JSON.stringify({
          ...JSON.parse(localStorage.settings),
          borderColor: event.target.value,
        })
      }}
    />
  </label>
</div>
