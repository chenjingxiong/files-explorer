import { ref, computed } from "vue";
import { useI18n } from "vue-i18n";
import { useFileStore } from "@/stores/fileStore";
import { useSelectionStore } from "@/stores/selectionStore";
import { useSettingsStore } from "@/stores/settingsStore";
import { useViewStore } from "@/stores/viewStore";
import { getClipboardInfo, getWindowsContextMenu } from "@/utils/tauri";
import type { ContextMenuOption, WinContextItem } from "@/types";

// ── Menu icons (14x14 viewBox SVG paths) ──
const I = {
  newFolder: `<svg viewBox="0 0 14 14"><path d="M2 3.5a1 1 0 011-1h2.5l1.2 1.5H11a1 1 0 011 1V11a1 1 0 01-1 1H3a1 1 0 01-1-1V3.5z" fill="none" stroke="currentColor" stroke-width="1"/><path d="M6.5 7v3M5 8.5h3" stroke="currentColor" stroke-width="1" stroke-linecap="round"/></svg>`,
  newFile: `<svg viewBox="0 0 14 14"><path d="M3.5 2h4.5L11 4.5V11.5a.5.5 0 01-.5.5h-7a.5.5 0 01-.5-.5V2.5a.5.5 0 01.5-.5z" fill="none" stroke="currentColor" stroke-width="1"/><path d="M8 2v2.5a.5.5 0 00.5.5H11" fill="none" stroke="currentColor" stroke-width="1"/><path d="M6.5 7v3M5 8.5h3" stroke="currentColor" stroke-width="1" stroke-linecap="round"/></svg>`,
  open: `<svg viewBox="0 0 14 14"><path d="M2 4.5a1 1 0 011-1h2.5l1.2 1.5H11a1 1 0 011 1V11a1 1 0 01-1 1H3a1 1 0 01-1-1V4.5z" fill="none" stroke="currentColor" stroke-width="1"/></svg>`,
  cut: `<svg viewBox="0 0 14 14"><circle cx="4.5" cy="3.5" r="1.5" fill="none" stroke="currentColor" stroke-width="1"/><circle cx="4.5" cy="10.5" r="1.5" fill="none" stroke="currentColor" stroke-width="1"/><path d="M5.5 4.5L10 9m0-4L5.5 9.5" stroke="currentColor" stroke-width="1" stroke-linecap="round"/></svg>`,
  copy: `<svg viewBox="0 0 14 14"><rect x="3.5" y="1.5" width="7" height="9" rx="1" fill="none" stroke="currentColor" stroke-width="1"/><rect x="1.5" y="3.5" width="7" height="9" rx="1" fill="var(--bg-secondary)" stroke="currentColor" stroke-width="1"/></svg>`,
  paste: `<svg viewBox="0 0 14 14"><path d="M4.5 2h5a.5.5 0 01.5.5V4H4V2.5a.5.5 0 01.5-.5z" fill="none" stroke="currentColor" stroke-width="1"/><rect x="3" y="3.5" width="8" height="9" rx="1" fill="none" stroke="currentColor" stroke-width="1"/></svg>`,
  copyPath: `<svg viewBox="0 0 14 14"><path d="M4 3h2l1 1.5H11a1 1 0 011 1V10a1 1 0 01-1 1H4a1 1 0 01-1-1V4a1 1 0 011-1z" fill="none" stroke="currentColor" stroke-width="1"/><path d="M6 7l1 1 3-2.5" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/></svg>`,
  delete: `<svg viewBox="0 0 14 14"><path d="M3 3.5h8M5.5 3V2.5a.5.5 0 01.5-.5h2a.5.5 0 01.5.5V3" fill="none" stroke="currentColor" stroke-width="1"/><path d="M4 3.5v8a1 1 0 001 1h4a1 1 0 001-1v-8" fill="none" stroke="currentColor" stroke-width="1"/></svg>`,
  rename: `<svg viewBox="0 0 14 14"><path d="M3 11l3-7.5L7.5 7l-3 4H3zm4.5-8l1.5 1.5M10 2l2 2-4 4-2.5-.5L6 4.5 10 2z" fill="none" stroke="currentColor" stroke-width="1" stroke-linejoin="round"/></svg>`,
  terminal: `<svg viewBox="0 0 14 14"><rect x="1.5" y="2.5" width="11" height="9" rx="1" fill="none" stroke="currentColor" stroke-width="1"/><path d="M4 5l2 2-2 2M7 9h3" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"/></svg>`,
  split: `<svg viewBox="0 0 14 14"><rect x="1.5" y="1.5" width="5" height="5" rx=".5" fill="none" stroke="currentColor" stroke-width="1"/><rect x="7.5" y="1.5" width="5" height="5" rx=".5" fill="none" stroke="currentColor" stroke-width="1"/><rect x="1.5" y="7.5" width="5" height="5" rx=".5" fill="none" stroke="currentColor" stroke-width="1"/><rect x="7.5" y="7.5" width="5" height="5" rx=".5" fill="none" stroke="currentColor" stroke-width="1"/></svg>`,
  splitLeft: `<svg viewBox="0 0 14 14"><rect x="1.5" y="1.5" width="4" height="11" rx=".5" fill="none" stroke="currentColor" stroke-width="1"/><path d="M7 7h4M9 5l2 2-2 2" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"/></svg>`,
  splitRight: `<svg viewBox="0 0 14 14"><rect x="8.5" y="1.5" width="4" height="11" rx=".5" fill="none" stroke="currentColor" stroke-width="1"/><path d="M7 7H3M5 5L3 7l2 2" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"/></svg>`,
  splitUp: `<svg viewBox="0 0 14 14"><rect x="1.5" y="1.5" width="11" height="4" rx=".5" fill="none" stroke="currentColor" stroke-width="1"/><path d="M7 7v4M5 9l2-2 2 2" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"/></svg>`,
  splitDown: `<svg viewBox="0 0 14 14"><rect x="1.5" y="8.5" width="11" height="4" rx=".5" fill="none" stroke="currentColor" stroke-width="1"/><path d="M7 7V3M5 5l2 2 2-2" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"/></svg>`,
  properties: `<svg viewBox="0 0 14 14"><circle cx="7" cy="7" r="5.5" fill="none" stroke="currentColor" stroke-width="1"/><path d="M7 5v3.5M7 10.5v.01" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/></svg>`,
  showInExplorer: `<svg viewBox="0 0 14 14"><path d="M8 2h4v4M6 8l6-6M4 3H3a1 1 0 00-1 1v7a1 1 0 001 1h7a1 1 0 001-1v-1" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"/></svg>`,
  selectAll: `<svg viewBox="0 0 14 14"><rect x="2" y="2" width="10" height="10" rx="1.5" fill="none" stroke="currentColor" stroke-width="1" stroke-dasharray="2 1.5"/><path d="M4.5 7l2 2 3.5-4" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/></svg>`,
  refresh: `<svg viewBox="0 0 14 14"><path d="M2 7a5 5 0 015-5 4.9 4.9 0 013.5 1.5M12 7a5 5 0 01-5 5 4.9 4.9 0 01-3.5-1.5" fill="none" stroke="currentColor" stroke-width="1"/><path d="M9.5 2V4.5H12M4.5 12V9.5H2" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"/></svg>`,
  preview: `<svg viewBox="0 0 14 14"><circle cx="5.5" cy="5.5" r="3.5" fill="none" stroke="currentColor" stroke-width="1.2"/><path d="M8 8l4.5 4.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/></svg>`,
  compress: `<svg viewBox="0 0 14 14"><rect x="2" y="1.5" width="10" height="11" rx="1" fill="none" stroke="currentColor" stroke-width="1"/><path d="M5 4h4M5 6h4M7 8v3" stroke="currentColor" stroke-width="1" stroke-linecap="round"/></svg>`,
  extract: `<svg viewBox="0 0 14 14"><rect x="2" y="1.5" width="10" height="11" rx="1" fill="none" stroke="currentColor" stroke-width="1"/><path d="M5 10h4" stroke="currentColor" stroke-width="1" stroke-linecap="round"/><path d="M7 2v5M5.5 5.5L7 7l1.5-1.5" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"/></svg>`,
  windows: `<svg viewBox="0 0 14 14"><path d="M1.5 2.5L6 1.8v4.7H1.5V2.5zm5 4.7H12.5V1L6.5 1.8v5.4zm-5 1.3H6v4.7L1.5 11.5V8.5zm5 0h6v5.5l-6-.8V8.5z" fill="currentColor"/></svg>`,
  windowsApp: `<svg viewBox="0 0 14 14"><rect x="1.5" y="1.5" width="11" height="11" rx="1.5" fill="none" stroke="currentColor" stroke-width="1.1"/><path d="M4 4.5h6M4 7h6M4 9.5h4" stroke="currentColor" stroke-width="1" stroke-linecap="round"/></svg>`,
  sort: `<svg viewBox="0 0 14 14"><path d="M2.5 3h6M2.5 6h4.5M2.5 9h3M2.5 12h1.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><path d="M10 9l2 2 2-2M12 4v7" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/></svg>`,
  pin: `<svg viewBox="0 0 14 14"><path d="M9.5 1.5l3 3-1.5 1.5-1-1-3 3 1.5 1.5-1.2 1.2-3-3L1.5 10.5V8l2.8-2.8 1.5 1.5 3-3-1-1 1.7-1.7z" fill="none" stroke="currentColor" stroke-width="1.1" stroke-linejoin="round"/></svg>`,
};

// ── Background Cached Windows Context Menu Items ──
export const cachedWinItems = ref<WinContextItem[]>([]);
let winPreloaded = false;

export async function preloadWinContextMenu() {
  if (winPreloaded) return;
  try {
    const items = await getWindowsContextMenu();
    cachedWinItems.value = items || [];
    winPreloaded = true;
  } catch (e) {
    console.warn("Failed to load Windows context items:", e);
  }
}

// Background pre-fetch immediately
preloadWinContextMenu();

export function useContextMenu() {
  const { t } = useI18n();
  const store = useFileStore();
  const sel = useSelectionStore();
  const view = useViewStore();

  const isMac =
    typeof navigator !== "undefined" && /Mac/.test(navigator.platform);

  const showContextMenu = ref(false);
  const contextMenuPos = ref({ x: 0, y: 0 });
  const sidebarContextPath = ref("");
  const rightClickedPath = ref("");
  const hasClipboardContent = ref(false);

  async function openContextMenu(x: number, y: number) {
    contextMenuPos.value = { x, y };
    showContextMenu.value = true;
    try {
      const info = await getClipboardInfo();
      hasClipboardContent.value = info.paths.length > 0;
    } catch {
      hasClipboardContent.value = false;
    }
  }

  function closeContextMenu() {
    showContextMenu.value = false;
  }

  const items = computed<ContextMenuOption[]>(() => {
    const hasSelection = sel.selectedFiles.size > 0;
    const singleSelection = sel.selectedFiles.size === 1;
    const result: ContextMenuOption[] = [];

    const sk = (ctrlKey: string, cmdKey: string) => t(isMac ? cmdKey : ctrlKey);

    if (!store.currentPath) {
      result.push({
        label: t("contextMenu.open"),
        action: "open",
        icon: I.open,
      });
      return result;
    }

    // Determine target path & target type (file vs directory vs background)
    let targetPath = "";
    let targetType: "file" | "directory" | "background" = "background";
    if (hasSelection) {
      targetPath = [...sel.selectedFiles][0] || "";
      const file = store.files.find((f) => f.path === targetPath);
      if (file) {
        targetType = file.is_dir ? "directory" : "file";
      } else {
        targetType = targetPath.includes(".") ? "file" : "directory";
      }
    } else {
      targetPath = store.currentPath || "";
      targetType = "background";
    }

    // ── Open / Show / Preview / Terminal ──
    if (hasSelection) {
      result.push(
        { label: t("contextMenu.open"), action: "open", icon: I.open },
        { label: "", action: "", separator: true },
      );
    }
    result.push(
      {
        label: isMac
          ? t("contextMenu.showInFinder")
          : t("contextMenu.showInExplorer"),
        action: "showInExplorer",
        icon: I.showInExplorer,
      },
      { label: "", action: "", separator: true },
      {
        label: t("contextMenu.openInPreviewWindow"),
        action: "openInPreviewWindow",
        icon: I.preview,
      },
      {
        label: t("contextMenu.openInTerminal"),
        action: "openInTerminal",
        icon: I.terminal,
      },
    );

    // ── Clipboard ──
    if (hasSelection) {
      result.push(
        { label: "", action: "", separator: true },
        {
          label: t("contextMenu.cut"),
          action: "cut",
          icon: I.cut,
          shortcut: sk("shortcuts.ctrlX", "shortcuts.cmdX"),
        },
        {
          label: t("contextMenu.copy"),
          action: "copy",
          icon: I.copy,
          shortcut: sk("shortcuts.ctrlC", "shortcuts.cmdC"),
        },
      );
    }
    result.push({
      label: t("contextMenu.paste"),
      action: "paste",
      icon: I.paste,
      shortcut: sk("shortcuts.ctrlV", "shortcuts.cmdV"),
      disabled: !hasClipboardContent.value,
    });
    if (hasSelection) {
      result.push({
        label: t("contextMenu.copyPath"),
        action: "copyPath",
        icon: I.copyPath,
      });
    }

    // ── Delete / Rename ──
    if (hasSelection) {
      result.push(
        { label: "", action: "", separator: true },
        {
          label: t("contextMenu.delete"),
          action: "delete",
          icon: I.delete,
          shortcut: t("shortcuts.del"),
          children: [
            { label: t("dialogs.delete"), action: "delete", icon: I.delete },
            {
              label: t("dialogs.deletePermanent"),
              action: "deletePermanent",
              icon: I.delete,
            },
          ],
        },
      );
      if (singleSelection) {
        result.push({
          label: t("contextMenu.rename"),
          action: "rename",
          icon: I.rename,
          shortcut: t("shortcuts.f2"),
        });
      }
    }

    // ── 5. Quick Access (添加到快捷访问 / 从快捷访问中移除) ──
    if (targetPath) {
      const settings = useSettingsStore();
      const normPath = targetPath.replace(/\\/g, "/");
      const isBookmarked = settings.bookmarks.some(
        (b) => b.path.replace(/\\/g, "/") === normPath,
      );
      result.push(
        { label: "", action: "", separator: true },
        {
          label: isBookmarked
            ? t("contextMenu.removeFromQuickAccess")
            : t("contextMenu.addToQuickAccess"),
          action: isBookmarked ? "removeFromQuickAccess" : "addToQuickAccess",
          icon: I.pin,
        },
      );
    }

    // ── 4. Sort By Submenu (以xxx排序，并记住偏好) ──
    result.push(
      { label: "", action: "", separator: true },
      {
        label: t("contextMenu.sortBy"),
        action: "sortByMenu",
        icon: I.sort,
        children: [
          {
            label: t("fileList.name"),
            action: "sort:name",
            checked: view.sortField === "name",
          },
          {
            label: t("fileList.dateModified"),
            action: "sort:modified",
            checked: view.sortField === "modified",
          },
          {
            label: t("fileList.dateCreated"),
            action: "sort:created",
            checked: view.sortField === "created",
          },
          {
            label: t("fileList.size"),
            action: "sort:size",
            checked: view.sortField === "size",
          },
          {
            label: t("fileList.type"),
            action: "sort:type",
            checked: view.sortField === "type",
          },
          { label: "", action: "", separator: true },
          {
            label: t("contextMenu.sortAsc"),
            action: "sort_dir:asc",
            checked: view.sortAsc,
          },
          {
            label: t("contextMenu.sortDesc"),
            action: "sort_dir:desc",
            checked: !view.sortAsc,
          },
        ],
      },
    );

    // ── 3. Windows System Context Menu (从预存好的后台上下文立即构建) ──
    if (cachedWinItems.value.length > 0) {
      const matchingWinItems = cachedWinItems.value.filter((item) => {
        if (item.applies_to === "all") return true;
        if (targetType === "background") return item.applies_to === "background";
        if (targetType === "directory") return item.applies_to === "directory";
        if (targetType === "file") return item.applies_to === "file";
        return false;
      });

      if (matchingWinItems.length > 0) {
        const winChildren: ContextMenuOption[] = matchingWinItems.map((item) => ({
          label: item.title,
          action: `win_verb:${item.id}`,
          icon: I.windowsApp,
        }));

        result.push(
          { label: "", action: "", separator: true },
          {
            label: t("contextMenu.windowsMenu"),
            action: "windowsMenu",
            icon: I.windows,
            children: winChildren,
          },
        );
      }
    }

    // ── Compress / Extract ──
    if (hasSelection) {
      result.push(
        { label: "", action: "", separator: true },
        {
          label: t("contextMenu.compress"),
          action: "compress",
          icon: I.compress,
        },
      );
    }
    if (hasSelection) {
      const archiveExts = ["zip", "tar", "gz", "tgz", "7z", "rar"];
      const allArchive = [...sel.selectedFiles].every((p) => {
        const ext = p.split(".").pop()?.toLowerCase() || "";
        return archiveExts.includes(ext);
      });
      if (allArchive) {
        result.push({
          label: t("contextMenu.extract"),
          action: "extract",
          icon: I.extract,
        });
      }
    }

    // ── Split ──
    result.push(
      { label: "", action: "", separator: true },
      {
        label: t("split.label"),
        action: "split",
        icon: I.split,
        children: [
          { label: t("split.left"), action: "splitLeft", icon: I.splitLeft },
          { label: t("split.right"), action: "splitRight", icon: I.splitRight },
          { label: t("split.up"), action: "splitUp", icon: I.splitUp },
          { label: t("split.down"), action: "splitDown", icon: I.splitDown },
        ],
      },
    );

    // ── Select / Refresh / New ──
    result.push(
      { label: "", action: "", separator: true },
      {
        label: t("contextMenu.selectAll"),
        action: "selectAll",
        icon: I.selectAll,
        shortcut: sk("shortcuts.ctrlA", "shortcuts.cmdA"),
      },
      { label: "", action: "", separator: true },
      {
        label: t("contextMenu.refresh"),
        action: "refresh",
        icon: I.refresh,
        shortcut: t("shortcuts.f5"),
      },
      { label: "", action: "", separator: true },
      {
        label: t("contextMenu.newFolder"),
        action: "newFolder",
        icon: I.newFolder,
        shortcut: sk("shortcuts.ctrlShiftN", "shortcuts.cmdShiftN"),
      },
      {
        label: t("contextMenu.newFile"),
        action: "newFile",
        icon: I.newFile,
        shortcut: sk("shortcuts.ctrlN", "shortcuts.cmdN"),
      },
    );

    // ── Properties (last) ──
    if (singleSelection) {
      result.push(
        { label: "", action: "", separator: true },
        {
          label: t("contextMenu.properties"),
          action: "properties",
          icon: I.properties,
        },
      );
    }

    return result;
  });

  return {
    showContextMenu,
    contextMenuPos,
    sidebarContextPath,
    rightClickedPath,
    contextMenuItems: items,
    openContextMenu,
    closeContextMenu,
  };
}
