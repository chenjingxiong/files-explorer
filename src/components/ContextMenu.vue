<template>
    <div
        ref="menuRef"
        class="context-menu"
        :style="{ left: adjX + 'px', top: adjY + 'px' }"
        @click.stop
    >
        <template v-for="(item, i) in items" :key="i">
            <div v-if="item.separator" class="context-menu-separator"></div>
            <div
                v-else-if="item.children && item.children.length > 0"
                class="context-menu-item has-sub"
                @mouseenter="subId = i"
                @mouseleave="onSubLeave"
            >
                <div class="item-left">
                    <span v-if="item.checked" class="check-mark">✓</span>
                    <span
                        v-else-if="item.icon"
                        class="menu-icon"
                        v-html="item.icon"
                    ></span>
                    <span v-else class="menu-icon-placeholder"></span>
                    <span class="item-label">{{ item.label }}</span>
                </div>
                <span class="arrow">▶</span>
                <div
                    v-if="subId === i"
                    class="submenu"
                    :class="{ 'submenu-left': openSubLeft }"
                    @mouseenter="subId = i"
                >
                    <template v-for="(child, ci) in item.children" :key="ci">
                        <div
                            v-if="child.separator"
                            class="context-menu-separator"
                        ></div>
                        <div
                            v-else
                            class="context-menu-item"
                            :class="{ disabled: child.disabled }"
                            @click.stop="!child.disabled && onChildClick(child.action)"
                        >
                            <div class="item-left">
                                <span v-if="child.checked" class="check-mark">✓</span>
                                <span
                                    v-else-if="child.icon"
                                    class="menu-icon"
                                    v-html="child.icon"
                                ></span>
                                <span v-else class="menu-icon-placeholder"></span>
                                <span class="item-label">{{ child.label }}</span>
                            </div>
                            <span v-if="child.shortcut" class="shortcut">{{
                                child.shortcut
                            }}</span>
                        </div>
                    </template>
                </div>
            </div>
            <div
                v-else
                class="context-menu-item"
                :class="{ disabled: item.disabled }"
                @click="!item.disabled && onItemClick(item.action)"
            >
                <div class="item-left">
                    <span v-if="item.checked" class="check-mark">✓</span>
                    <span
                        v-else-if="item.icon"
                        class="menu-icon"
                        v-html="item.icon"
                    ></span>
                    <span v-else class="menu-icon-placeholder"></span>
                    <span class="item-label">{{ item.label }}</span>
                </div>
                <span v-if="item.shortcut" class="shortcut">{{
                    item.shortcut
                }}</span>
            </div>
        </template>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import type { ContextMenuOption } from "@/types";

const props = defineProps<{
    x: number;
    y: number;
    items: ContextMenuOption[];
}>();
const emit = defineEmits<{ close: []; action: [action: string] }>();
const menuRef = ref<HTMLElement | null>(null);
const subId = ref<number | null>(null);

// ── Esc 关闭菜单 ──
function onKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
        emit("close");
    }
}
onMounted(() => window.addEventListener("keydown", onKeydown));
onUnmounted(() => window.removeEventListener("keydown", onKeydown));

const adjX = computed(() => Math.min(props.x, window.innerWidth - 240));
const adjY = computed(() =>
    Math.min(props.y, window.innerHeight - (props.items.length * 32 + 20)),
);
const openSubLeft = computed(() => adjX.value > window.innerWidth - 420);

function onItemClick(action: string) {
    emit("action", action);
}
function onChildClick(action: string) {
    subId.value = null;
    emit("action", action);
}
function onSubLeave() {
    setTimeout(() => {
        subId.value = null;
    }, 120);
}
</script>

<style scoped>
.context-menu {
    position: fixed;
    z-index: 1000;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 4px;
    min-width: 210px;
    box-shadow: 0 8px 32px var(--shadow);
}
.context-menu-item {
    display: flex;
    align-items: center;
    gap: 8px;
    justify-content: space-between;
    padding: 6px 12px;
    border-radius: 4px;
    cursor: pointer;
    font-size: var(--font-size-base);
    position: relative;
    user-select: none;
}
.item-left {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
    min-width: 0;
}
.item-label {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}
.check-mark {
    width: 14px;
    height: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 13px;
    font-weight: bold;
    color: var(--accent);
    flex-shrink: 0;
}
.menu-icon-placeholder {
    width: 14px;
    height: 14px;
    flex-shrink: 0;
}
.menu-icon {
    width: 14px;
    height: 14px;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
}
.menu-icon :deep(svg) {
    width: 14px;
    height: 14px;
}
.context-menu-item:hover {
    background: var(--bg-hover);
}
.context-menu-item.disabled {
    color: var(--text-muted);
    cursor: default;
}
.context-menu-item.disabled:hover {
    background: transparent;
}
.context-menu-item .shortcut {
    color: var(--text-muted);
    font-size: var(--font-size-sm);
    margin-left: 12px;
}
.arrow {
    font-size: var(--font-size-xs);
    color: var(--text-muted);
    margin-left: 8px;
}
.context-menu-separator {
    height: 1px;
    background: var(--border);
    margin: 4px 8px;
}
.submenu {
    position: absolute;
    left: 100%;
    top: -4px;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 4px;
    min-width: 180px;
    max-height: 400px;
    overflow-y: auto;
    box-shadow: 0 4px 16px var(--shadow);
    z-index: 1001;
}
.submenu-left {
    left: auto !important;
    right: 100% !important;
}
</style>
