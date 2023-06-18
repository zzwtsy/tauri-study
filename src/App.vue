<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
</script>

<template>
  <div class="container">
    <el-container>
      <el-aside width="150px">
        <Menu style="height: 100vh;" />
      </el-aside>
      <el-main>
        <Greet />
      </el-main>
    </el-container>
  </div>
</template>

<style scoped>

</style>
