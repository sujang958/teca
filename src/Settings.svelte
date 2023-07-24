<div class="flex flex-col">
  <p class="text-3xl font-semibold">Settings</p>
  <div class="py-3" />
  <label class="text-sm flex flex-col gap-y-2">
    Keys
    <input
      type="text"
      class="rounded-lg bg-neutral-950 border border-neutral-700 px-1.5 outline-none w-min text-base py-1"
      value={localStorage.getItem("keys")}
      on:input={(event) => {
        if (!(event.target instanceof HTMLInputElement)) return
        localStorage.setItem("keys", event.target.value)
      }}
    />
  </label>
</div>
