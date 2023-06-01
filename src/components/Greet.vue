<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const gistId = ref();

async function greet() {
  let id = gistId.value;
  console.log(typeof id);
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("gist_id", { id: gistId.value as number });
}
</script>

<template>
  <div class="card">
    <input
      id="greet-input"
      v-model.number="gistId"
      placeholder="Enter a gist id..."
    />
    <button type="button" @click="greet()">Greet</button>
  </div>

  <p>{{ greetMsg }}</p>
</template>
