<template>
  <el-header class="header">
    <el-row :gutter="10" justify="center" align="middle" style="height: 100%">
      <!-- 项目名称输入框 -->
      <el-col :span="3">
        <el-input placeholder="项目名称"></el-input>
      </el-col>
      <!-- 选择日期范围 -->
      <el-col :span="5.5">
        <el-date-picker
          v-model="dateRange"
          type="daterange"
          unlink-panels
          range-separator="To"
          start-placeholder="开始日期"
          end-placeholder="结束日期"
          :shortcuts="shortcuts"
          value-format="YYYY-MM-DD"
        />
      </el-col>
      <!-- 搜索按钮 -->
      <el-col :span="3">
        <el-button @click="getAllWakatimeData()" type="primary">搜索</el-button>
      </el-col>
    </el-row>
  </el-header>
  <el-main>
    <el-table :data="wakatimeData" border style="width: 100%">
      <el-table-column prop="date" label="Date" width="180" />
      <el-table-column prop="text" label="Text" width="180" />
      <el-table-column prop="timezone" label="Timezone" />
    </el-table>
  </el-main>
</template>

<script setup lang="ts">
import { ref } from "vue";
import * as Api from "../ts/Api";
import { invoke } from "@tauri-apps/api/tauri";

const wakatimeData = ref();

async function getAllWakatimeData() {
  console.log("start");
  const data: any = await invoke("get_all_wakatime_data");
  console.table(data);
  wakatimeData.value = data.range;
  console.table(wakatimeData.value)
}

// 日期选择器代码开始
const dateRange = ref("");

const shortcuts = [
  {
    text: "过去一周",
    value: () => {
      const end = new Date();
      const start = new Date();
      start.setTime(start.getTime() - 3600 * 1000 * 24 * 7);
      return [start, end];
    },
  },
  {
    text: "过去一个月",
    value: () => {
      const end = new Date();
      const start = new Date();
      start.setTime(start.getTime() - 3600 * 1000 * 24 * 30);
      return [start, end];
    },
  },
  {
    text: "过去三个月",
    value: () => {
      const end = new Date();
      const start = new Date();
      start.setTime(start.getTime() - 3600 * 1000 * 24 * 90);
      return [start, end];
    },
  },
];
// 日期选择器代码结束
</script>

<style lang="scss">
.header {
  border-bottom: 1px solid var(--el-menu-border-color);
  padding: 0;
}
</style>
