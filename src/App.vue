<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import { onMounted, ref, onBeforeMount, nextTick, onUnmounted } from "vue";
import { createWidget } from "./core/map";
import CreditVue from "./components/Credit.vue";
import CesiumBtnVue from "./components/CesiumBtn.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Typhoon, Header } from "./core/definition";
import * as typhoonState from "./core/typhoon";
import * as dataState from "./core/dataState";

const conatiner = ref();
const credit = ref();
const headers = ref<Header[]>([]);
const initData = async () => {
  const typhoonsData: Typhoon[] = JSON.parse(await invoke("get_typhoon_data"));
  dataState.init(typhoonsData);
};
const sync = async () => {
  await invoke("sync_typhoon_data");
  await initData();
  const rawData = dataState.getRaw();
  await typhoonState.refresh(rawData);
  headers.value = dataState.getHeaders();
};

const triggerMapTime = (header: Header) => {
  const typhoon = dataState.getTyphoon(header.InternationalNumberID);
  typhoonState.select(dataState.dateParse(typhoon.points[0].Time));
};

onBeforeMount(async () => {
  await initData();
});

onMounted(async () => {
  const widget = await createWidget(conatiner.value, credit.value);
  const dataSetupInterval = setInterval(async () => {
    const rawData = dataState.getRaw();
    if (!rawData) {
      return;
    }
    headers.value = dataState.getHeaders();
    await typhoonState.init(widget, rawData);
    clearInterval(dataSetupInterval);
  }, 1000);
});
onUnmounted(async () => {
  typhoonState.clear();
});
</script>

<template>
  <div class="container" ref="conatiner">
    <div class="v-toolbar">
      <CesiumBtnVue @click="sync">
        <v-icon icon="mdi-autorenew"></v-icon>
      </CesiumBtnVue>
    </div>
    <div class="c-timeline">
      <v-timeline class="overflow-x-hidden" side="end" density="compact">
        <v-timeline-item v-for="header in headers" size="x-small" density="compact">
          <div class="c-btn disable-select" @click="triggerMapTime(header)">
            {{ header.last?.toJSON().split(":")[0] }} <br />
            {{ header.Name || "No Name" }} ( {{ header.InternationalNumberID }} )
          </div>
        </v-timeline-item>
      </v-timeline>
    </div>
    <div class="v-credit disable-select" ref="credit">
      <div ref="creditViewport">
        <CreditVue />
      </div>
    </div>
  </div>
</template>

<style scoped>
.disable-select {
  user-select: none; /* supported by Chrome and Opera */
  -webkit-user-select: none; /* Safari */
  -khtml-user-select: none; /* Konqueror HTML */
  -moz-user-select: none; /* Firefox */
  -ms-user-select: none; /* Internet Explorer/Edge */
}

.container {
  z-index: 0;
}

.v-credit {
  z-index: 1;
  position: absolute;
  bottom: 1.6rem;
  right: 0;
}

.v-toolbar {
  z-index: 1;
  position: absolute;
  top: 0;
  right: 0;
  margin: 0.16em;
}

.c-timeline {
  z-index: 1;
  position: absolute;
  height: 85vh;
  max-height: 90vh;
  max-width: 20vw;
  margin-bottom: 0.6em;
  top: 3rem;
  left: 0;
}

.c-btn:hover {
  cursor: pointer;
  color: burlywood;
}
</style>
