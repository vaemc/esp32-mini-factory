<template>
  <div>
    <a-modal
      v-model:visible="firmwareModal.visible"
      :title="firmwareModal.title"
      @ok="firmwareModalOk"
    >
      <a-form :label-col="{ span: 4 }">
        <a-form-item label="路径" name="description">
          <a-input-search
            enter-button="选择固件"
            v-model:value="firmware.path"
            @search="openFileDialog(firmware)"
          />
        </a-form-item>

        <a-form-item label="烧录地址" name="address">
          <a-input v-model:value="firmware.address" />
        </a-form-item>

        <a-form-item label="固件名" name="filename">
          <a-input placeholder="可选" v-model:value="firmware.filename" />
        </a-form-item>

        <a-form-item label="描述" name="description">
          <a-input placeholder="可选" v-model:value="firmware.description" />
        </a-form-item>
      </a-form>
    </a-modal>

    <div style="padding: 5px">
      <a-card size="small" title="烧录选项" style="width: 100%">
        <template #extra>
          <a-space :size="5">
            <a-button type="primary" @click="firmwareModal.visible = true"
              >添加固件</a-button
            >
          </a-space>
        </template>

        <a-table
          :pagination="false"
          :dataSource="firmwareList"
          :columns="columns"
          size="small"
          class="scroll"
          :scroll="{ y: 500 }"
        >
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'action'">
              <a @click="editData(record)">编辑</a> |
              <a @click="removeData(record)">删除</a>
            </template>
          </template>
        </a-table>
      </a-card>
    </div>
    <div style="padding: 5px">
      <a-card size="small" title="串口列表" style="width: 100%">
        <template #extra>
          <div>
            <a-space :size="5">
              <a-button type="primary">刷新串口列表</a-button>
              <a-select
                placeholder="选择芯片"
                :options="chipTypeOptionList"
                style="width: 130px"
              >
              </a-select>
              <a-button type="primary">开始烧录</a-button>
            </a-space>
          </div>
        </template>
        <div style="display: flex; flex-wrap: wrap">
          <div
            v-for="item in serialPortList"
            style="flex-basis: 50%; padding: 5px"
          >
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
      </a-card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { message } from "ant-design-vue";
import { Firmware } from "../model";
async function getSerialPortList() {
  let data = (await invoke("get_serial_port_list")) as string;
  return JSON.parse(data);
}

const firmware = ref({} as Firmware);
const firmwareList = ref([] as Firmware[]);
const firmwareModal = ref({ visible: false, title: "添加固件" });
const firmwareModalOk = () => {
  firmwareModal.value.visible = false;
  firmwareList.value.push(firmware.value);
};

const openFileDialog = async (obj: any) => {
  const selected = await open({
    directory: false,
    multiple: false,
  });
  if (!Array.isArray(selected) && selected !== null) {
    obj.path = selected;
  }
};

const chipTypeList = ref([
  "ESP32",
  "ESP32C2",
  "ESP32C3",
  "ESP32C6",
  "ESP32S2",
  "ESP32S3",
  "ESP32H2",
  "ESP8266",
]);

const chipTypeOptionList = ref(
  chipTypeList.value.map((item) => {
    return {
      value: item,
      label: item,
    };
  })
);

const columns = ref([
  {
    title: "路径",
    dataIndex: "path",
    key: "path",
    ellipsis: true,
  },
  {
    title: "烧录地址",
    dataIndex: "address",
    key: "address",
  },
  {
    title: "固件名",
    dataIndex: "filename",
    key: "filename",
  },
  {
    title: "描述",
    dataIndex: "description",
    key: "description",
  },
  {
    title: "操作",
    key: "action",
  },
]);

const serialPortList = ref(await getSerialPortList());

const removeData = (item: Firmware) => {
  firmwareList.value = firmwareList.value.filter(
    (x: Firmware) => x.path != item.path
  );
  message.success("删除成功！");
};
const editData = (item: Firmware) => {};

console.log(serialPortList);
</script>
