<script setup lang="ts">
import { useTemplateRef } from 'vue';
import { toPixel } from '@tb-dev/utils';
import { useHeight } from '@tb-dev/vue';
import {
  Button,
  InputNumber,
  Label,
  ScrollArea,
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from '@tb-dev/vue-components';

const config = defineModel<Config>({ required: true });

const contentEl = useTemplateRef('contentEl');
const contentHeight = useHeight(contentEl);
</script>

<template>
  <Sheet>
    <SheetTrigger as-child>
      <Button size="sm">Options</Button>
    </SheetTrigger>
    <SheetContent>
      <SheetHeader>
        <SheetTitle>Options</SheetTitle>
      </SheetHeader>

      <div ref="contentEl" class="size-full p-4">
        <ScrollArea :style="{ height: toPixel(contentHeight - 80) }">
          <div class="flex flex-col items-start gap-3 pr-4 pb-4 pl-2">
            <Label>
              <span>Min. Level</span>
              <InputNumber
                v-model="config.minLevel"
                :min="1"
                :max="config.maxLevel - 1"
                :step="1"
              />
            </Label>
            <Label>
              <span>Max. Level</span>
              <InputNumber
                v-model="config.maxLevel"
                :min="config.minLevel + 1"
                :max="30"
                :step="1"
              />
            </Label>
            <Label>
              <span>Wood</span>
              <InputNumber v-model="config.wood" :min="0" :max="1" :step="0.05" />
            </Label>
            <Label>
              <span>Stone</span>
              <InputNumber v-model="config.stone" :min="0" :max="1" :step="0.05" />
            </Label>
            <Label>
              <span>Iron</span>
              <InputNumber v-model="config.iron" :min="0" :max="1" :step="0.05" />
            </Label>
            <Label>
              <span>Min. Cost</span>
              <InputNumber
                v-model="config.minCost"
                :min="0"
                :max="config.maxCost - 1"
                :step="1_000"
              />
            </Label>
            <Label>
              <span>Max. Cost</span>
              <InputNumber
                v-model="config.maxCost"
                :min="config.minCost + 1"
                :max="1_000_000_000"
                :step="1_000"
              />
            </Label>
            <Label>
              <span>Maintenance</span>
              <InputNumber v-model="config.maintenance" :min="0" :max="1" :step="0.001" />
            </Label>
            <Label>
              <span>Min. Workforce</span>
              <InputNumber
                v-model="config.minWorkforce"
                :min="0"
                :max="config.maxWorkforce - 1"
                :step="10"
              />
            </Label>
            <Label>
              <span>Max. Workforce</span>
              <InputNumber
                v-model="config.maxWorkforce"
                :min="config.minWorkforce + 1"
                :max="10_000"
                :step="10"
              />
            </Label>
            <Label>
              <span>Min. Production</span>
              <InputNumber
                v-model="config.minProduction"
                :min="0"
                :max="config.maxProduction - 1"
                :step="100"
              />
            </Label>
            <Label>
              <span>Max. Production</span>
              <InputNumber
                v-model="config.maxProduction"
                :min="config.minProduction + 1"
                :max="100_000"
                :step="100"
              />
            </Label>
            <Label>
              <span>Min. Capacity</span>
              <InputNumber
                v-model="config.minCapacity"
                :min="0"
                :max="config.maxCapacity - 1"
                :step="10_000"
              />
            </Label>
            <Label>
              <span>Max. Capacity</span>
              <InputNumber
                v-model="config.maxCapacity"
                :min="config.minCapacity + 1"
                :max="100_000_000"
                :step="10_000"
              />
            </Label>
            <Label>
              <span>Min. Value</span>
              <InputNumber
                v-model="config.minCustom"
                :min="0"
                :max="config.maxCustom - 1"
                :step="1"
              />
            </Label>
            <Label>
              <span>Max. Value</span>
              <InputNumber
                v-model="config.maxCustom"
                :min="config.minCustom + 1"
                :max="100_000_000"
                :step="1"
              />
            </Label>
          </div>
        </ScrollArea>
        <div></div>
      </div>
    </SheetContent>
  </Sheet>
</template>
