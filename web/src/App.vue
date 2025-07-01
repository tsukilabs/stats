<script setup lang="ts">
import Sheet from '@/components/Sheet.vue';
import { useColorMode } from '@vueuse/core';
import { localRef, useHeight } from '@tb-dev/vue';
import { calc, Stats, Config as WasmConfig } from 'core';
import { shallowRef, useTemplateRef, watchEffect } from 'vue';
import { Table, TableCell, TableHead, TableRow } from '@tb-dev/vue-components';

const mainEl = useTemplateRef('mainEl');
const mainHeight = useHeight(mainEl);

const wasmConfig = new WasmConfig();
const stats = shallowRef<Stats[]>([]);

const config = localRef<Config>('nil:config', {
  maxLevel: 30,
  wood: 0.3,
  stone: 0.4,
  iron: 0.3,
  maintenance: 0.005,
  cost: 100_000,
  costGrowth: 0.2,
  workforce: 150,
  workforceGrowth: 0.2,
  production: 3_600,
  productionGrowth: 0.2,
  capacity: 400_000,
  capacityGrowth: 0.2,
});

watchEffect(() => {
  wasmConfig.max_level = config.value.maxLevel;
  wasmConfig.wood = config.value.wood;
  wasmConfig.stone = config.value.stone;
  wasmConfig.iron = config.value.iron;
  wasmConfig.maintenance = config.value.maintenance;
  wasmConfig.cost = config.value.cost;
  wasmConfig.cost_growth = config.value.costGrowth;
  wasmConfig.workforce = config.value.workforce;
  wasmConfig.workforce_growth = config.value.workforceGrowth;
  wasmConfig.production = config.value.production;
  wasmConfig.production_growth = config.value.productionGrowth;
  wasmConfig.capacity = config.value.capacity;
  wasmConfig.capacity_growth = config.value.capacityGrowth;

  stats.value = calc(wasmConfig);
});

useColorMode({
  initialValue: 'dark',
  writeDefaults: true,
});
</script>

<template>
  <div class="fixed inset-0 overflow-hidden select-none">
    <header class="flex h-[48px] items-center justify-between px-4">
      <div class="text-lg font-semibold">Stats Calculator</div>
      <Sheet v-model="config" />
    </header>
    <main ref="mainEl" class="h-[calc(100vh-48px)] w-full">
      <Table
        :height="mainHeight - 30"
        width="100vw"
        class="px-4 pb-4"
        header-class="sticky top-0 z-50"
      >
        <template #header>
          <TableRow class="bg-background hover:bg-background">
            <TableHead v-if="config.maxLevel > 1" class="w-16">
              <span>Level</span>
            </TableHead>
            <TableHead v-if="config.cost && config.costGrowth && config.wood">
              <span>Wood</span>
            </TableHead>
            <TableHead v-if="config.cost && config.costGrowth && config.stone">
              <span>Stone</span>
            </TableHead>
            <TableHead v-if="config.cost && config.costGrowth && config.iron">
              <span>Iron</span>
            </TableHead>
            <TableHead v-if="config.cost && config.costGrowth">
              <span>Cost</span>
            </TableHead>
            <TableHead v-if="config.cost && config.maintenance">
              <span>Maintenance</span>
            </TableHead>
            <TableHead v-if="config.workforce && config.workforceGrowth">
              <span>Workforce</span>
            </TableHead>
            <TableHead v-if="config.production && config.productionGrowth">
              <span>Production</span>
            </TableHead>
            <TableHead v-if="config.capacity && config.capacityGrowth">
              <span>Capacity</span>
            </TableHead>
          </TableRow>
        </template>

        <TableRow v-for="result of stats" :key="result.level">
          <TableCell v-if="config.maxLevel > 1">
            {{ result.level }}
          </TableCell>
          <TableCell v-if="config.cost && config.costGrowth && config.wood">
            {{ result.wood }}
          </TableCell>
          <TableCell v-if="config.cost && config.costGrowth && config.stone">
            {{ result.stone }}
          </TableCell>
          <TableCell v-if="config.cost && config.costGrowth && config.iron">
            {{ result.iron }}
          </TableCell>
          <TableCell v-if="config.cost && config.costGrowth">
            {{ result.total_cost }}
          </TableCell>
          <TableCell v-if="config.cost && config.maintenance">
            {{ result.maintenance }}
          </TableCell>
          <TableCell v-if="config.workforce && config.workforceGrowth">
            {{ result.workforce }}
          </TableCell>
          <TableCell v-if="config.production && config.productionGrowth">
            {{ result.production }}
          </TableCell>
          <TableCell v-if="config.capacity && config.capacityGrowth">
            {{ result.capacity }}
          </TableCell>
        </TableRow>
      </Table>
    </main>
  </div>
</template>
