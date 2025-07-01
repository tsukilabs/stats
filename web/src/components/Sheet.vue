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
              <span>Level</span>
              <InputNumber v-model="config.maxLevel" :min="1" :max="30" :step="1" />
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
              <span>Cost</span>
              <InputNumber v-model="config.cost" :min="0" :max="1_000_000_000" :step="1_000" />
            </Label>
            <Label>
              <span>Cost Growth</span>
              <InputNumber v-model="config.costGrowth" :min="0" :max="1" :step="0.05" />
            </Label>
            <Label>
              <span>Maintenance</span>
              <InputNumber v-model="config.maintenance" :min="0" :max="1" :step="0.001" />
            </Label>
            <Label>
              <span>Workforce</span>
              <InputNumber v-model="config.workforce" :min="0" :max="10_000" :step="10" />
            </Label>
            <Label>
              <span>Workforce Growth</span>
              <InputNumber v-model="config.workforceGrowth" :min="0" :max="1" :step="0.05" />
            </Label>
            <Label>
              <span>Production</span>
              <InputNumber v-model="config.production" :min="0" :max="100_000" :step="100" />
            </Label>
            <Label>
              <span>Production Growth</span>
              <InputNumber v-model="config.productionGrowth" :min="0" :max="1" :step="0.05" />
            </Label>
            <Label>
              <span>Capacity</span>
              <InputNumber v-model="config.capacity" :min="0" :max="100_000_000" :step="10_000" />
            </Label>
            <Label>
              <span>Capacity Growth</span>
              <InputNumber v-model="config.capacityGrowth" :min="0" :max="1" :step="0.05" />
            </Label>
          </div>
        </ScrollArea>
        <div></div>
      </div>
    </SheetContent>
  </Sheet>
</template>
