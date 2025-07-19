<script setup lang="ts">
import { useHeight } from '@tb-dev/vue';
import Sheet from '@/components/Sheet.vue';
import { useColorMode } from '@vueuse/core';
import { calc, Stats, Config as WasmConfig } from 'core';
import { ref, shallowRef, useTemplateRef, watchEffect } from 'vue';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@tb-dev/vue-components';

const mainEl = useTemplateRef('main');
const mainHeight = useHeight(mainEl);

const wasmConfig = new WasmConfig();
const stats = shallowRef<Stats[]>([]);

const config = ref<Config>({
  minLevel: 1,
  maxLevel: 30,
  minCost: 1_000,
  maxCost: 100_000,
  minWorkforce: 1,
  maxWorkforce: 150,
  minProduction: 30,
  maxProduction: 2400,
  minCapacity: 1_000,
  maxCapacity: 400_000,
  minCustom: 1,
  maxCustom: 100_000,
  wood: 0.3,
  stone: 0.4,
  iron: 0.3,
  maintenance: 0.005,
});

watchEffect(() => {
  wasmConfig.min_level = config.value.minLevel;
  wasmConfig.max_level = config.value.maxLevel;
  wasmConfig.min_cost = config.value.minCost;
  wasmConfig.max_cost = config.value.maxCost;
  wasmConfig.min_workforce = config.value.minWorkforce;
  wasmConfig.max_workforce = config.value.maxWorkforce;
  wasmConfig.min_production = config.value.minProduction;
  wasmConfig.max_production = config.value.maxProduction;
  wasmConfig.min_capacity = config.value.minCapacity;
  wasmConfig.max_capacity = config.value.maxCapacity;
  wasmConfig.wood = config.value.wood;
  wasmConfig.stone = config.value.stone;
  wasmConfig.iron = config.value.iron;
  wasmConfig.maintenance = config.value.maintenance;

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
    <main ref="main" class="h-[calc(100vh-48px)] w-full">
      <Table
        :height="mainHeight - 30"
        width="100vw"
        class="px-4 pb-4"
        header-class="sticky top-0 z-50"
      >
        <TableHeader>
          <TableRow class="bg-background hover:bg-background">
            <TableHead v-if="config.maxLevel > 1" class="w-16">
              <span>Level</span>
            </TableHead>
            <TableHead v-if="config.minCost && config.maxCost && config.wood">
              <span>Wood</span>
            </TableHead>
            <TableHead v-if="config.minCost && config.maxCost && config.stone">
              <span>Stone</span>
            </TableHead>
            <TableHead v-if="config.minCost && config.maxCost && config.iron">
              <span>Iron</span>
            </TableHead>
            <TableHead v-if="config.minCost && config.maxCost">
              <span>Cost</span>
            </TableHead>
            <TableHead v-if="config.minCost && config.maxCost && config.maintenance">
              <span>Maintenance</span>
            </TableHead>
            <TableHead v-if="config.minWorkforce && config.maxWorkforce">
              <span>Workforce</span>
            </TableHead>
            <TableHead v-if="config.minProduction && config.maxProduction">
              <span>Production</span>
            </TableHead>
            <TableHead v-if="config.minCapacity && config.maxCapacity">
              <span>Capacity</span>
            </TableHead>
            <TableHead v-if="config.minCustom && config.maxCustom">
              <span>Value (custom)</span>
            </TableHead>
          </TableRow>
        </TableHeader>

        <TableBody>
          <TableRow v-for="result of stats" :key="result.level">
            <TableCell v-if="config.maxLevel > 1">
              {{ result.level }}
            </TableCell>
            <TableCell v-if="config.minCost && config.maxCost && config.wood">
              {{ result.wood }}
            </TableCell>
            <TableCell v-if="config.minCost && config.maxCost && config.stone">
              {{ result.stone }}
            </TableCell>
            <TableCell v-if="config.minCost && config.maxCost && config.iron">
              {{ result.iron }}
            </TableCell>
            <TableCell v-if="config.minCost && config.maxCost">
              {{ result.cost }}
            </TableCell>
            <TableCell v-if="config.minCost && config.maxCost && config.maintenance">
              {{ result.maintenance }}
            </TableCell>
            <TableCell v-if="config.minWorkforce && config.maxWorkforce">
              {{ result.workforce }}
            </TableCell>
            <TableCell v-if="config.minProduction && config.maxProduction">
              {{ result.production }}
            </TableCell>
            <TableCell v-if="config.minCapacity && config.maxCapacity">
              {{ result.capacity }}
            </TableCell>
            <TableCell v-if="config.minCustom && config.maxCustom">
              {{ result.custom }}
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </main>
  </div>
</template>
