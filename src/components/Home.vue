<template>
  <div>
    <div>asdasd</div>
    <div style="display: flex; flex-wrap: wrap">
      <div v-for="item in serialPortList" style="flex-basis: 50%; padding: 5px">
        <a-card :title="item" style="" size="small">
          <template #extra>
            <a-checkbox checked="true">烧录</a-checkbox></template
          >
     
          <a-input-search
            size="small"
            style="margin: 2px"
            placeholder=""
            suffix="0x"
            enter-button="选择"
          />
          <a-input-search
            size="small"
            style="margin: 2px"
            placeholder=""
            suffix="0x"
            enter-button="选择"
          />
          <a-input-search
            size="small"
            style="margin: 2px"
            placeholder=""
            suffix="0x"
            enter-button="选择"
          />
          <a-input-search
            size="small"
            style="margin: 2px"
            placeholder=""
            suffix="0x"
            enter-button="选择"
          />
          <a-input-search
            size="small"
            style="margin: 2px"
            placeholder=""
            suffix="0x"
            enter-button="选择"
          />
        </a-card>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
async function getSerialPortList() {
  let data = (await invoke("get_serial_port_list")) as string;
  return JSON.parse(data);
}

const serialPortList = ref(await getSerialPortList());

console.log(serialPortList);
</script>
