<script setup lang="ts">
import { useTemplateRef } from 'vue';
import { toPixel } from '@tb-dev/utils';
import { useHeight } from '@tb-dev/vue';
import {
  Button,
  Label,
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
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
              <NumberField v-model="config.minLevel" :min="0" :max="100" class="w-full">
                <NumberFieldContent>
                  <NumberFieldDecrement />
                  <NumberFieldInput class="dark:bg-input/40" />
                  <NumberFieldIncrement />
                </NumberFieldContent>
              </NumberField>
            </Label>

            <Label>
              <span>Max. Level</span>
              <NumberField v-model="config.maxLevel" :min="0" :max="100" class="w-full">
                <NumberFieldContent>
                  <NumberFieldDecrement />
                  <NumberFieldInput class="dark:bg-input/40" />
                  <NumberFieldIncrement />
                </NumberFieldContent>
              </NumberField>
            </Label>

            <Label>
              <span>Wood</span>
              <NumberField v-model="config.wood" :min="0" :max="1" :step="0.01" class="w-full">
                <NumberFieldContent>
                  <NumberFieldDecrement />
                  <NumberFieldInput class="dark:bg-input/40" />
                  <NumberFieldIncrement />
                </NumberFieldContent>
              </NumberField>
            </Label>

            <Label>
              <span>Stone</span>
              <NumberField v-model="config.stone" :min="0" :max="1" :step="0.01" class="w-full">
                <NumberFieldContent>
                  <NumberFieldDecrement />
                  <NumberFieldInput class="dark:bg-input/40" />
                  <NumberFieldIncrement />
                </NumberFieldContent>
              </NumberField>
            </Label>

            <Label>
              <span>Iron</span>
              <NumberField v-model="config.iron" :min="0" :max="1" :step="0.01" class="w-full">
                <NumberFieldContent>
                  <NumberFieldDecrement />
                  <NumberFieldInput class="dark:bg-input/40" />
                  <NumberFieldIncrement />
                </NumberFieldContent>
              </NumberField>
            </Label>

            <Label>
              <span>Min. Cost</span>
              <NumberField v-model="config.minCost" :min="0" :max="1_000_000_000" class="w-full">
                <NumberFieldContent>
                  <NumberFieldDecrement />
                  <NumberFieldInput class="dark:bg-input/40" />
                  <NumberFieldIncrement />
                </NumberFieldContent>
              </NumberField>
            </Label>

            <Label>
              <span>Max. Cost</span>
              <NumberField v-model="config.maxCost" :min="0" :max="1_000_000_000" class="w-full">
                <NumberFieldContent>
                  <NumberFieldDecrement />
                  <NumberFieldInput class="dark:bg-input/40" />
                  <NumberFieldIncrement />
                </NumberFieldContent>
              </NumberField>
            </Label>

            <Label>
              <span>Maintenance</span>
              <NumberField
                v-model="config.maintenance"
                :min="0"
                :max="1"
                :step="0.001"
                class="w-full"
              >
                <NumberFieldContent>
                  <NumberFieldDecrement />
                  <NumberFieldInput class="dark:bg-input/40" />
                  <NumberFieldIncrement />
                </NumberFieldContent>
              </NumberField>
            </Label>

            <Label>
              <span>Min. Workforce</span>
              <NumberField v-model="config.minWorkforce" :min="0" :max="10_000" class="w-full">
                <NumberFieldContent>
                  <NumberFieldDecrement />
                  <NumberFieldInput class="dark:bg-input/40" />
                  <NumberFieldIncrement />
                </NumberFieldContent>
              </NumberField>
            </Label>

            <Label>
              <span>Max. Workforce</span>
              <NumberField v-model="config.maxWorkforce" :min="0" :max="10_000" class="w-full">
                <NumberFieldContent>
                  <NumberFieldDecrement />
                  <NumberFieldInput class="dark:bg-input/40" />
                  <NumberFieldIncrement />
                </NumberFieldContent>
              </NumberField>
            </Label>

            <Label>
              <span>Min. Production</span>
              <NumberField v-model="config.minProduction" :min="0" :max="100_000" class="w-full">
                <NumberFieldContent>
                  <NumberFieldDecrement />
                  <NumberFieldInput class="dark:bg-input/40" />
                  <NumberFieldIncrement />
                </NumberFieldContent>
              </NumberField>
            </Label>

            <Label>
              <span>Max. Production</span>
              <NumberField v-model="config.maxProduction" :min="0" :max="100_000" class="w-full">
                <NumberFieldContent>
                  <NumberFieldDecrement />
                  <NumberFieldInput class="dark:bg-input/40" />
                  <NumberFieldIncrement />
                </NumberFieldContent>
              </NumberField>
            </Label>

            <Label>
              <span>Min. Capacity</span>
              <NumberField v-model="config.minCapacity" :min="0" :max="100_000_000" class="w-full">
                <NumberFieldContent>
                  <NumberFieldDecrement />
                  <NumberFieldInput class="dark:bg-input/40" />
                  <NumberFieldIncrement />
                </NumberFieldContent>
              </NumberField>
            </Label>

            <Label>
              <span>Max. Capacity</span>
              <NumberField v-model="config.maxCapacity" :min="0" :max="100_000_000" class="w-full">
                <NumberFieldContent>
                  <NumberFieldDecrement />
                  <NumberFieldInput class="dark:bg-input/40" />
                  <NumberFieldIncrement />
                </NumberFieldContent>
              </NumberField>
            </Label>

            <Label>
              <span>Min. Value</span>
              <NumberField v-model="config.minCustom" class="w-full">
                <NumberFieldContent>
                  <NumberFieldDecrement />
                  <NumberFieldInput class="dark:bg-input/40" />
                  <NumberFieldIncrement />
                </NumberFieldContent>
              </NumberField>
            </Label>

            <Label>
              <span>Max. Value</span>
              <NumberField v-model="config.maxCustom" class="w-full">
                <NumberFieldContent>
                  <NumberFieldDecrement />
                  <NumberFieldInput class="dark:bg-input/40" />
                  <NumberFieldIncrement />
                </NumberFieldContent>
              </NumberField>
            </Label>
          </div>
        </ScrollArea>
        <div></div>
      </div>
    </SheetContent>
  </Sheet>
</template>
