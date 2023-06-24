<template>
  <el-menu router default-active="/home" class="menu">
    <el-menu-item index="/home" class="menu-item">
      <el-icon><Icon-ri-bar-chart-box-line /></el-icon>
      <template #title> 首页 </template>
    </el-menu-item>
    <el-menu-item index="/data" class="menu-item">
      <el-icon><Icon-ri-database-line /></el-icon>
      <template #title> 数据 </template>
    </el-menu-item>
    <el-menu-item index="/setting" class="menu-item">
      <el-icon><Icon-ri-settings-3-line /></el-icon>
      <template #title> 设置 </template>
    </el-menu-item>
  </el-menu>
</template>

<script setup lang="ts"></script>

<style scoped lang="scss">
.menu {
  padding: 5px 0;
}
.menu-item {
  box-sizing: border-box;
  margin: 5px 10px 0 10px;
  border-radius: 5px;
  &:first-child {
    margin-top: 0;
  }
  &:hover {
    background-color: transparent;
    background-color: #eee;
  }
}
</style>
