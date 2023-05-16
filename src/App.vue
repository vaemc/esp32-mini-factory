<template>
  <a-config-provider :locale="locale">
    <Suspense>
      <Home />
    </Suspense>
  </a-config-provider>
</template>
<script setup lang="ts">
import Home from "./components/Home.vue";
import zhCN from "ant-design-vue/es/locale/zh_CN";
import dayjs from "dayjs";
import "dayjs/locale/zh-cn";

dayjs.locale("zh-cn");

const locale = zhCN;
</script>
<style>
*::-webkit-scrollbar-track {
  -webkit-box-shadow: inset 0 0 6px rgba(0, 0, 0, 0.3);
  background-color: #141414;
}

*::-webkit-scrollbar-thumb {
  background-color: #000000;
  border: 2px solid #555555;
}

*::-webkit-scrollbar {
  width: 10px;
  height: 10px;
  background-color: #141414;
}
</style>
