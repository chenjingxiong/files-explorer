<template>
    <div class="dialog-overlay" @click.self="$emit('close')">
        <div class="settings-dialog">
            <div class="settings-header">
                <h2>
                    <svg
                        viewBox="0 0 18 18"
                        width="16"
                        height="16"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="1.3"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        style="vertical-align: middle; margin-right: 6px"
                    >
                        <circle cx="9" cy="9" r="2.5" />
                        <path
                            d="M9 1.5v2M9 14.5v2M1.5 9h2M14.5 9h2M3.3 3.3l1.4 1.4M13.3 13.3l1.4 1.4M3.3 14.7l1.4-1.4M13.3 4.7l1.4-1.4"
                        />
                    </svg>
                    {{ t("settings.title") }}
                </h2>
                <button class="icon-btn close-btn" @click="$emit('close')">
                    <svg viewBox="0 0 20 20" fill="none">
                        <path
                            d="M5 5l10 10M15 5L5 15"
                            stroke="currentColor"
                            stroke-width="1.5"
                            stroke-linecap="round"
                        />
                    </svg>
                </button>
            </div>

            <div class="settings-layout">
                <!-- Tab sidebar -->
                <div class="settings-tabs">
                    <button
                        v-for="tab in tabs"
                        :key="tab.id"
                        class="settings-tab"
                        :class="{ active: activeTab === tab.id }"
                        @click="activeTab = tab.id"
                    >
                        <span class="tab-icon" v-html="tab.icon"></span>
                        <span class="tab-label">{{ t(tab.labelKey) }}</span>
                    </button>
                </div>

                <!-- Tab content -->
                <div class="settings-content">
                    <!-- Appearance -->
                    <div v-if="activeTab === 'appearance'" class="tab-panel">
                        <div class="setting-group">
                            <div class="group-title">
                                {{ t("settings.theme") }}
                            </div>
                            <div class="theme-grid">
                                <button
                                    v-for="opt in themeOptions"
                                    :key="opt.value"
                                    class="theme-card"
                                    :class="{
                                        active: settings.theme === opt.value,
                                    }"
                                    @click="settings.setTheme(opt.value)"
                                >
                                    <div
                                        class="theme-bars"
                                        :style="{ background: opt.preview.bg }"
                                    >
                                        <div
                                            class="tb-bar"
                                            :style="{
                                                background: opt.preview.sidebar,
                                            }"
                                        ></div>
                                        <div class="tb-spacer"></div>
                                        <div
                                            class="tb-dot"
                                            :style="{
                                                background: opt.preview.accent,
                                            }"
                                        ></div>
                                    </div>
                                    <span class="theme-label">{{
                                        t(opt.labelKey)
                                    }}</span>
                                </button>
                            </div>
                        </div>

                                                <div class="setting-group">
                            <div class="font-setting-header">
                                <div class="group-title">
                                    {{ t("settings.fontSize") }}
                                </div>
                                <span class="font-badge">{{ settings.fontSize }}px</span>
                            </div>

                            <div class="font-slider-container">
                                <span class="font-slider-label">10px</span>
                                <input
                                    type="range"
                                    min="10"
                                    max="24"
                                    step="1"
                                    :value="settings.fontSize"
                                    class="font-slider-range"
                                    @input="handleFontSizeInput"
                                />
                                <span class="font-slider-label">24px</span>
                            </div>

                            <div class="font-presets">
                                <button
                                    v-for="preset in [11, 12, 13, 14, 16, 18, 20, 22, 24]"
                                    :key="preset"
                                    class="font-preset-chip"
                                    :class="{ active: settings.fontSize === preset }"
                                    @click="settings.setFontSize(preset)"
                                >
                                    {{ preset }}px
                                </button>
                            </div>

                            <!-- Live preview card -->
                            <div class="font-preview-box">
                                <div class="font-preview-top">
                                    <span class="font-preview-badge">{{ t("settings.fontPreviewTitle") || "布局与字号实时预览" }}</span>
                                    <span class="font-preview-current">{{ settings.fontSize }}px</span>
                                </div>
                                <div class="font-preview-list">
                                    <div class="preview-row">
                                        <svg viewBox="0 0 16 16" width="16" height="16" class="preview-icon">
                                            <path d="M1 3.5A1.5 1.5 0 0 1 2.5 2h2.764c.958 0 1.76.56 2.311 1.184l.732.816H13.5A1.5 1.5 0 0 1 15 5.5v7a1.5 1.5 0 0 1-1.5 1.5h-11A1.5 1.5 0 0 1 1 12.5v-9z" fill="#F5C542"/>
                                        </svg>
                                        <span class="preview-name">Documents</span>
                                        <span class="preview-type">{{ t("fileList.folder") || "文件夹" }}</span>
                                        <span class="preview-date">2026-10-29 18:00</span>
                                        <span class="preview-size">—</span>
                                    </div>
                                    <div class="preview-row">
                                        <svg viewBox="0 0 16 16" width="16" height="16" class="preview-icon">
                                            <path d="M14 4.5V14a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V2a2 2 0 0 1 2-2h5.5L14 4.5zm-3 0A1.5 1.5 0 0 1 9.5 3V1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1V4.5h-2z" fill="#89B4FA"/>
                                        </svg>
                                        <span class="preview-name">Annual_Financial_Report_2026.docx</span>
                                        <span class="preview-type">Word 文档</span>
                                        <span class="preview-date">2026-10-28 14:20</span>
                                        <span class="preview-size">3.8 MB</span>
                                    </div>
                                    <div class="preview-row">
                                        <svg viewBox="0 0 16 16" width="16" height="16" class="preview-icon">
                                            <path d="M14 4.5V14a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V2a2 2 0 0 1 2-2h5.5L14 4.5z" fill="#A6E3A1"/>
                                        </svg>
                                        <span class="preview-name">main.rs</span>
                                        <span class="preview-type">Rust Source</span>
                                        <span class="preview-date">2026-10-29 17:45</span>
                                        <span class="preview-size">18.2 KB</span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="setting-group">
                            <div class="group-title">
                                {{ t("settings.iconTheme") }}
                            </div>
                            <div class="theme-grid">
                                <button
                                    class="theme-card"
                                    :class="{
                                        active: settings.iconTheme === 'fluent',
                                    }"
                                    @click="settings.setIconTheme('fluent')"
                                >
                                    <div class="icon-preview fluent-preview">
                                        <div class="ip-file ip-blue">JS</div>
                                        <div class="ip-file ip-green">VU</div>
                                        <div class="ip-folder"></div>
                                    </div>
                                    <span class="theme-label">{{
                                        t("settings.fluent")
                                    }}</span>
                                </button>
                                <button
                                    class="theme-card"
                                    :class="{
                                        active:
                                            settings.iconTheme === 'material',
                                    }"
                                    @click="settings.setIconTheme('material')"
                                >
                                    <div class="icon-preview material-preview">
                                        <div
                                            class="mp-file"
                                            style="background: #3178c6"
                                        >
                                            TS
                                        </div>
                                        <div
                                            class="mp-file"
                                            style="background: #41b883"
                                        >
                                            VU
                                        </div>
                                        <div
                                            class="mp-folder"
                                            style="background: #f6c23a"
                                        >
                                            FD
                                        </div>
                                    </div>
                                    <span class="theme-label">{{
                                        t("settings.material")
                                    }}</span>
                                    <span class="theme-label">{{
                                        t("settings.material")
                                    }}</span>
                                </button>
                                <button
                                    class="theme-card"
                                    :class="{
                                        active:
                                            settings.iconTheme ===
                                            'material-full',
                                    }"
                                    @click="
                                        settings.setIconTheme('material-full')
                                    "
                                >
                                    <div
                                        class="icon-preview material-full-preview"
                                    >
                                        <img
                                            src="/icons/typescript.svg"
                                            width="20"
                                            height="20"
                                        />
                                        <img
                                            src="/icons/vue.svg"
                                            width="20"
                                            height="20"
                                        />
                                        <img
                                            src="/icons/folder-src.svg"
                                            width="20"
                                            height="20"
                                        />
                                    </div>
                                    <span class="theme-label">{{
                                        t("settings.materialFull")
                                    }}</span>
                                </button>
                            </div>
                        </div>
                    </div>

                    <!-- Terminal -->
                    <div v-if="activeTab === 'terminal'" class="tab-panel">
                        <div class="setting-group">
                            <div class="group-title">
                                {{ t("settings.termType") }}
                            </div>
                            <div class="term-select-wrapper">
                                <button
                                    class="term-select-trigger"
                                    :class="{ open: termDropdownOpen }"
                                    @click="
                                        termDropdownOpen = !termDropdownOpen
                                    "
                                >
                                    <span class="term-select-icon">
                                        <svg
                                            viewBox="0 0 16 16"
                                            width="14"
                                            height="14"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="1.3"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                        >
                                            <rect
                                                x="1.5"
                                                y="2.5"
                                                width="13"
                                                height="11"
                                                rx="1.2"
                                            />
                                            <path
                                                d="M4.5 6l2.5 2-2.5 2M8 10h3.5"
                                            />
                                        </svg>
                                    </span>
                                    <span class="term-select-label">{{
                                        t(currentTermLabelKey)
                                    }}</span>
                                    <svg
                                        class="term-select-arrow"
                                        viewBox="0 0 12 12"
                                        width="10"
                                        height="10"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="1.5"
                                        stroke-linecap="round"
                                    >
                                        <path d="M3 4.5l3 3 3-3" />
                                    </svg>
                                </button>
                                <div
                                    v-if="termDropdownOpen"
                                    class="term-select-dropdown"
                                >
                                    <button
                                        v-for="opt in termOptions"
                                        :key="opt.value"
                                        class="term-select-item"
                                        :class="{
                                            active:
                                                settings.termEmulation ===
                                                opt.value,
                                        }"
                                        @mousedown.prevent="
                                            settings.setTermEmulation(
                                                opt.value,
                                            );
                                            termDropdownOpen = false;
                                        "
                                    >
                                        <span class="term-select-item-icon">
                                            <svg
                                                viewBox="0 0 16 16"
                                                width="14"
                                                height="14"
                                                fill="none"
                                                stroke="currentColor"
                                                stroke-width="1.3"
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                            >
                                                <rect
                                                    x="1.5"
                                                    y="2.5"
                                                    width="13"
                                                    height="11"
                                                    rx="1.2"
                                                />
                                                <path
                                                    d="M4.5 6l2.5 2-2.5 2M8 10h3.5"
                                                />
                                            </svg>
                                        </span>
                                        <span class="term-select-item-label">{{
                                            t(opt.labelKey)
                                        }}</span>
                                        <svg
                                            v-if="
                                                settings.termEmulation ===
                                                opt.value
                                            "
                                            class="term-select-item-check"
                                            viewBox="0 0 14 14"
                                            width="12"
                                            height="12"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                        >
                                            <path d="M2 7l3.5 3.5L12 3" />
                                        </svg>
                                    </button>
                                </div>
                            </div>
                        </div>

                        <!-- Font family -->
                        <div class="setting-group">
                            <div class="group-title">
                                {{ t("settings.termFontFamily") }}
                            </div>
                            <div class="term-select-wrapper">
                                <button
                                    class="term-select-trigger"
                                    :class="{ open: fontFamilyOpen }"
                                    @click="fontFamilyOpen = !fontFamilyOpen"
                                >
                                    <span class="term-select-label">{{
                                        t(
                                            termFontLabels[
                                                settings.termFontFamily
                                            ],
                                        )
                                    }}</span>
                                    <svg
                                        class="term-select-arrow"
                                        viewBox="0 0 12 12"
                                        width="10"
                                        height="10"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="1.5"
                                        stroke-linecap="round"
                                    >
                                        <path d="M3 4.5l3 3 3-3" />
                                    </svg>
                                </button>
                                <div
                                    v-if="fontFamilyOpen"
                                    class="term-select-dropdown"
                                >
                                    <button
                                        v-for="(
                                            labelKey, key
                                        ) in termFontLabels"
                                        :key="key"
                                        class="term-select-item"
                                        :class="{
                                            active:
                                                settings.termFontFamily === key,
                                        }"
                                        @mousedown.prevent="
                                            settings.setTermFontFamily(
                                                key as any,
                                            );
                                            fontFamilyOpen = false;
                                        "
                                    >
                                        <span class="term-select-item-label">{{
                                            t(labelKey)
                                        }}</span>
                                        <svg
                                            v-if="
                                                settings.termFontFamily === key
                                            "
                                            class="term-select-item-check"
                                            viewBox="0 0 14 14"
                                            width="12"
                                            height="12"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                        >
                                            <path d="M2 7l3.5 3.5L12 3" />
                                        </svg>
                                    </button>
                                    <div class="term-select-sep"></div>
                                    <button
                                        class="term-select-item"
                                        :class="{
                                            active:
                                                settings.termFontFamily ===
                                                'custom',
                                        }"
                                        @mousedown.prevent="
                                            settings.setTermFontFamily(
                                                'custom',
                                            );
                                            fontFamilyOpen = false;
                                        "
                                    >
                                        <span class="term-select-item-label"
                                            >✏️
                                            {{ t("settings.customFont") }}</span
                                        >
                                    </button>
                                </div>
                            </div>
                            <div
                                v-if="settings.termFontFamily === 'custom'"
                                class="term-custom-input"
                            >
                                <input
                                    class="term-input"
                                    :value="settings.termFontCustom"
                                    @input="
                                        (e) =>
                                            settings.setTermFontCustom(
                                                (e.target as HTMLInputElement)
                                                    .value,
                                            )
                                    "
                                    placeholder="'Fira Code', Consolas, monospace"
                                />
                            </div>
                        </div>

                        <!-- Font size -->
                        <div class="setting-group">
                            <div class="group-title">
                                {{ t("settings.termFontSize") }}
                            </div>
                            <div class="term-select-wrapper">
                                <button
                                    class="term-select-trigger"
                                    :class="{ open: fontSizeOpen }"
                                    @click="fontSizeOpen = !fontSizeOpen"
                                >
                                    <span class="term-select-label"
                                        >{{ settings.termFontSize }}px</span
                                    >
                                    <svg
                                        class="term-select-arrow"
                                        viewBox="0 0 12 12"
                                        width="10"
                                        height="10"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="1.5"
                                        stroke-linecap="round"
                                    >
                                        <path d="M3 4.5l3 3 3-3" />
                                    </svg>
                                </button>
                                <div
                                    v-if="fontSizeOpen"
                                    class="term-select-dropdown"
                                >
                                    <button
                                        v-for="opt in fontSizes"
                                        :key="opt"
                                        class="term-select-item"
                                        :class="{
                                            active:
                                                settings.termFontSize === opt,
                                        }"
                                        @mousedown.prevent="
                                            settings.setTermFontSize(opt);
                                            fontSizeOpen = false;
                                        "
                                    >
                                        <span class="term-select-item-label"
                                            >{{ opt }}px</span
                                        >
                                        <svg
                                            v-if="settings.termFontSize === opt"
                                            class="term-select-item-check"
                                            viewBox="0 0 14 14"
                                            width="12"
                                            height="12"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                        >
                                            <path d="M2 7l3.5 3.5L12 3" />
                                        </svg>
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Language -->
                    <div v-if="activeTab === 'language'" class="tab-panel">
                        <div class="setting-group">
                            <div class="group-title">
                                {{ t("settings.language") }}
                            </div>
                            <div class="lang-options">
                                <button
                                    v-for="opt in langOptions"
                                    :key="opt.value"
                                    class="lang-card"
                                    :class="{
                                        active: settings.locale === opt.value,
                                    }"
                                    @click="handleLocaleChange(opt.value)"
                                >
                                    <span class="lang-flag">{{
                                        opt.flag
                                    }}</span>
                                    <div class="lang-info">
                                        <span class="lang-name">{{
                                            opt.name
                                        }}</span>
                                        <span class="lang-native">{{
                                            opt.native
                                        }}</span>
                                    </div>
                                    <svg
                                        v-if="settings.locale === opt.value"
                                        class="check-icon"
                                        viewBox="0 0 20 20"
                                        fill="none"
                                    >
                                        <path
                                            d="M4 10l4 4 8-8"
                                            stroke="var(--accent)"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                        />
                                    </svg>
                                </button>
                            </div>
                        </div>
                    </div>

                    <!-- About -->
                    <div v-if="activeTab === 'about'" class="tab-panel">
                        <div class="about-hero">
                            <img
                                class="about-icon"
                                src="/icon.png"
                                alt="Files Explorer"
                            />
                            <div class="about-name">{{ $t("app.title") }}</div>
                            <div class="about-version">
                                v{{ APP_VERSION }}
                            </div>
                            <div class="about-desc">
                                {{ t("settings.description") }}
                            </div>
                        </div>
                        <button
                            class="about-update-btn"
                            :disabled="updateCheckState === 'checking' || updateCheckState === 'downloading'"
                            @click="manualCheckUpdate"
                        >
                            <svg
                                v-if="updateCheckState === 'checking'"
                                class="spinner"
                                viewBox="0 0 14 14"
                                width="14"
                                height="14"
                            >
                                <circle cx="7" cy="7" r="5" fill="none" stroke="currentColor" stroke-width="1.5" stroke-dasharray="24 8" />
                            </svg>
                            <svg v-else-if="updateCheckState === 'up-to-date'" viewBox="0 0 14 14" width="14" height="14">
                                <circle cx="7" cy="7" r="6" fill="none" stroke="currentColor" stroke-width="1.3" />
                                <path d="M4 7l2.5 2.5L10 5.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round" />
                            </svg>
                            <svg v-else viewBox="0 0 14 14" width="14" height="14" fill="none">
                                <path d="M7 2v3M7 9v3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
                                <circle cx="7" cy="11" r="1.5" stroke="currentColor" stroke-width="3" stroke-dasharray="0 8" transform="rotate(45 7 7)" opacity="0" />
                                <path d="M7 2a5 5 0 015 5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
                                <circle cx="7" cy="7" r="4" fill="none" stroke="currentColor" stroke-width="1.2" stroke-dasharray="4 21" />
                            </svg>
                            {{ updateCheckLabel }}
                        </button>
                        <div class="about-pillars">
                            <span class="pillar-chip" :title="$t('settings.pillarLightweightTooltip')">
                                <svg viewBox="0 0 14 14" width="13" height="13" fill="none">
                                    <path d="M7 2L3 7l4 5 4-5-4-5z" stroke="currentColor" stroke-width="1.1" stroke-linejoin="round" />
                                    <path d="M4 7h6" stroke="currentColor" stroke-width="1.1" stroke-linecap="round" />
                                </svg>
                                {{ $t("settings.pillarLightweight") }}
                            </span>
                            <span class="pillar-chip" :title="$t('settings.pillarFastTooltip')">
                                <svg viewBox="0 0 14 14" width="13" height="13" fill="none">
                                    <path d="M6 3.5L11 7l-5 3.5v-7z" fill="currentColor" opacity="0.8" />
                                    <path d="M3 3.5l2.5 1.75v3.5L3 10.5v-7z" fill="currentColor" opacity="0.3" />
                                </svg>
                                {{ $t("settings.pillarFast") }}
                            </span>
                            <span class="pillar-chip" :title="$t('settings.pillarModernTooltip')">
                                <svg viewBox="0 0 14 14" width="13" height="13" fill="none">
                                    <rect x="2" y="2.5" width="10" height="9" rx="1.5" stroke="currentColor" stroke-width="1.1" />
                                    <path d="M5 7l1.5 1.5L9 5.5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round" stroke-linejoin="round" />
                                </svg>
                                {{ $t("settings.pillarModern") }}
                            </span>
                            <span class="pillar-chip" :title="$t('settings.pillarSmartTooltip')">
                                <svg viewBox="0 0 14 14" width="13" height="13" fill="none">
                                    <path d="M7 1.5L9.5 5l4 .5-3 3 .8 4.5L7 10.8 3.2 13l.8-4.5-3-3 4-.5L7 1.5z" stroke="currentColor" stroke-width="1.1" stroke-linejoin="round" />
                                </svg>
                                {{ $t("settings.pillarSmart") }}
                            </span>
                        </div>
                        <div class="about-links">
                            <button class="about-link-btn" @click="$emit('clearCache')">
                                <svg viewBox="0 0 14 14" width="14" height="14" fill="none">
                                    <path d="M2 4h10M5 4V2.5a1 1 0 011-1h2a1 1 0 011 1V4" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" />
                                    <path d="M3 4l.8 7.5a1 1 0 001 .5h4.4a1 1 0 001-.5L11 4" stroke="currentColor" stroke-width="1.2" />
                                </svg>
                                {{ $t("tray.clearCache") }}
                            </button>
                            <button class="about-link-btn" @click="openRepo">
                                <svg viewBox="0 0 14 14" width="14" height="14" fill="none">
                                    <path d="M5 2H3a1 1 0 00-1 1v8a1 1 0 001 1h8a1 1 0 001-1V9" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" />
                                    <path d="M13 1H9v4" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round" />
                                    <path d="M13 1L6 8" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" />
                                </svg>
                                {{ $t("settings.openSource") }}
                            </button>
                        </div>
                        <div class="about-footer">
                            Copyright © {{ new Date().getFullYear() }} Files Explorer
                        </div>
                    </div>

                    <!-- General -->
                    <div v-if="activeTab === 'general'" class="tab-panel">
                        <div class="setting-group">
                            <div class="group-title">
                                {{ t("settings.general") }}
                            </div>
                            <label
                                class="switch-row"
                                @click="
                                    settings.setAutoStart(!settings.autoStart)
                                "
                            >
                                <div class="switch-text">
                                    <span>{{ t("settings.autoStart") }}</span>
                                    <span class="switch-desc">{{
                                        t("settings.autoStartDesc")
                                    }}</span>
                                </div>
                                <div
                                    class="switch-track"
                                    :class="{ active: settings.autoStart }"
                                >
                                    <div class="switch-thumb" />
                                </div>
                            </label>
                            <label
                                class="switch-row"
                                @click="
                                    settings.setShowTray(!settings.showTray)
                                "
                            >
                                <div class="switch-text">
                                    <span>{{ t("settings.showTray") }}</span>
                                    <span class="switch-desc">{{
                                        t("settings.showTrayDesc")
                                    }}</span>
                                </div>
                                <div
                                    class="switch-track"
                                    :class="{ active: settings.showTray }"
                                >
                                    <div class="switch-thumb" />
                                </div>
                            </label>
                            <label
                                class="switch-row"
                                @click="
                                    settings.setQuitOnClose(
                                        !settings.quitOnClose,
                                    )
                                "
                            >
                                <div class="switch-text">
                                    <span>{{ t("settings.quitOnClose") }}</span>
                                    <span class="switch-desc">{{
                                        t("settings.quitOnCloseDesc")
                                    }}</span>
                                </div>
                                <div
                                    class="switch-track"
                                    :class="{ active: settings.quitOnClose }"
                                >
                                    <div class="switch-thumb" />
                                </div>
                            </label>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { useI18n } from "vue-i18n";
import { useSettingsStore } from "@/stores/settingsStore";
import { APP_VERSION } from "@/utils/version";
import { checkForUpdates, downloadSilently, subscribeUpdateTask } from "@/utils/updater";
import { useToast } from "@/composables/useToast";
import type {
    ThemeMode,
    FontSize,
    TermEmulation,
    TermFontSize,
    TermFontFamily,
} from "@/stores/settingsStore";

const { t, locale } = useI18n();
const settings = useSettingsStore();
const toast = useToast();
const activeTab = ref("general");
const termDropdownOpen = ref(false);

// ── 更新检查 ──
type UpdateCheckState = "idle" | "checking" | "downloading" | "up-to-date" | "error";
const updateCheckState = ref<UpdateCheckState>("idle");
const updateCheckLabel = computed(() => {
    switch (updateCheckState.value) {
        case "checking": return t("settings.checkingUpdate");
        case "downloading": return t("settings.downloadingUpdate");
        case "up-to-date": return t("settings.upToDate");
        case "error": return t("settings.updateCheckFailed");
        default: return t("settings.checkUpdate");
    }
});
const newVersion = ref<string | null>(null);

async function manualCheckUpdate() {
    if (updateCheckState.value === "checking" || updateCheckState.value === "downloading") return;
    updateCheckState.value = "checking";
    newVersion.value = null;
    try {
        const result = await checkForUpdates();
        if (result.available && result.update) {
            newVersion.value = result.update.version;
            updateCheckState.value = "downloading";
            const dl = downloadSilently();
            if (!dl.started) {
                updateCheckState.value = "idle";
                return;
            }
            // 监听：完成时清空按钮状态，UpdaterChecker 会自动弹出重启
            const unsub = subscribeUpdateTask((snapshot) => {
                if (snapshot.state === "ready_to_restart") {
                    unsub();
                    updateCheckState.value = "idle";
                }
                if (snapshot.state === "error") {
                    unsub();
                    updateCheckState.value = "error";
                    toast.show(snapshot.message || t("settings.updateCheckFailed"), true);
                    setTimeout(() => { if (updateCheckState.value === "error") updateCheckState.value = "idle"; }, 3000);
                }
            });
        } else if (result.errorCode === "NO_UPDATE") {
            updateCheckState.value = "up-to-date";
            toast.show(t("settings.alreadyLatest"));
            setTimeout(() => { if (updateCheckState.value === "up-to-date") updateCheckState.value = "idle"; }, 3000);
        } else {
            updateCheckState.value = "error";
            toast.show(result.message || t("settings.updateCheckFailed"), true);
            setTimeout(() => { if (updateCheckState.value === "error") updateCheckState.value = "idle"; }, 3000);
        }
    } catch (e) {
        updateCheckState.value = "error";
        toast.show(e instanceof Error ? e.message : t("settings.updateCheckFailed"), true);
        setTimeout(() => { if (updateCheckState.value === "error") updateCheckState.value = "idle"; }, 3000);
    }
}

function openRepo() {
    import("@tauri-apps/plugin-shell").then(({ open }) =>
        open("https://github.com/chaoyingLi/files-explorer")
    );
}


function handleFontSizeInput(e: Event) {
    const val = Number((e.target as HTMLInputElement).value);
    settings.setFontSize(val);
}
const fontSizeOpen = ref(false);
const fontFamilyOpen = ref(false);

defineEmits<{ close: []; clearCache: [] }>();

const tabs = [
    {
        id: "general",
        labelKey: "settings.tabGeneral",
        icon: `<svg viewBox="0 0 16 16" fill="none"><circle cx="8" cy="8" r="3" stroke="currentColor" stroke-width="1.3"/><path d="M8 1v3M8 12v3M1 8h3M12 8h3M3 3l2.5 2.5M10.5 10.5L13 13M3 13l2.5-2.5M10.5 5.5L13 3" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/></svg>`,
    },
    {
        id: "appearance",
        labelKey: "settings.tabAppearance",
        icon: `<svg viewBox="0 0 16 16" fill="none"><circle cx="8" cy="8" r="5" stroke="currentColor" stroke-width="1.3"/><path d="M8 3a5 5 0 000 10V3z" fill="currentColor" opacity="0.3"/></svg>`,
    },
    {
        id: "terminal",
        labelKey: "settings.tabTerminal",
        icon: `<svg viewBox="0 0 16 16" fill="none"><rect x="1.5" y="2.5" width="13" height="11" rx="1.5" stroke="currentColor" stroke-width="1.3"/><path d="M4.5 6l2.5 2-2.5 2M8 10h3" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/></svg>`,
    },
    {
        id: "language",
        labelKey: "settings.tabLanguage",
        icon: `<svg viewBox="0 0 16 16" fill="none"><circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.3"/><ellipse cx="8" cy="8" rx="3" ry="6" stroke="currentColor" stroke-width="1.1"/><path d="M2 8h12M8 2a9 9 0 010 12" stroke="currentColor" stroke-width="0.8"/></svg>`,
    },
    {
        id: "about",
        labelKey: "settings.tabAbout",
        icon: `<svg viewBox="0 0 16 16" fill="none"><circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.3"/><path d="M8 7.5v4M8 5v.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/></svg>`,
    },
];

const themeOptions: {
    value: ThemeMode;
    labelKey: string;
    preview: { bg: string; sidebar: string; accent: string };
}[] = [
    {
        value: "dark",
        labelKey: "settings.catppuccinDark",
        preview: { bg: "#1E1E2E", sidebar: "#11111B", accent: "#89B4FA" },
    },
    {
        value: "light",
        labelKey: "settings.catppuccinLight",
        preview: { bg: "#EFF1F5", sidebar: "#CCD0DA", accent: "#1E66F5" },
    },
    {
        value: "nord",
        labelKey: "settings.nord",
        preview: { bg: "#2E3440", sidebar: "#434C5E", accent: "#88C0D0" },
    },
    {
        value: "tokyo-night",
        labelKey: "settings.tokyoNight",
        preview: { bg: "#1A1B26", sidebar: "#1F202A", accent: "#7AA2F7" },
    },
    {
        value: "one-dark-pro",
        labelKey: "settings.oneDarkPro",
        preview: { bg: "#282C34", sidebar: "#1B1E24", accent: "#61AFEF" },
    },
    {
        value: "dracula",
        labelKey: "settings.dracula",
        preview: { bg: "#282A36", sidebar: "#1B1D26", accent: "#BD93F9" },
    },
    {
        value: "solarized-light",
        labelKey: "settings.solarizedLight",
        preview: { bg: "#FDF6E3", sidebar: "#E0D8C3", accent: "#268BD2" },
    },
];

const langOptions = [
    { value: "zh", name: "Chinese", native: "中文", flag: "🇨🇳" },
    { value: "en", name: "English", native: "English", flag: "🇺🇸" },
];

const fontSizeOptions: {
    value: FontSize;
    labelKey: string;
    sampleSize: string;
}[] = [
    { value: "small", labelKey: "settings.small", sampleSize: "14px" },
    { value: "medium", labelKey: "settings.medium", sampleSize: "18px" },
    { value: "large", labelKey: "settings.large", sampleSize: "24px" },
];

const termOptions: { value: TermEmulation; labelKey: string }[] = [
    { value: "xterm-256color", labelKey: "settings.xterm256" },
    { value: "xterm", labelKey: "settings.xterm" },
    { value: "xterm-direct", labelKey: "settings.xtermDirect" },
    { value: "vt100", labelKey: "settings.vt100" },
    { value: "linux", labelKey: "settings.linux" },
];

const currentTermLabelKey = computed(
    () =>
        termOptions.find((o) => o.value === settings.termEmulation)?.labelKey ||
        "settings.xterm256",
);

const fontSizes: TermFontSize[] = [10, 11, 12, 13, 14, 15, 16, 18, 20, 22, 24];

const termFontLabels: Record<TermFontFamily, string> = {
    system: "settings.systemDefault",
    "jetbrains-mono": "settings.jetbrainsMono",
    "cascadia-code": "settings.cascadiaCode",
    "fira-code": "settings.firaCode",
    "source-code-pro": "settings.sourceCodePro",
    hack: "settings.hack",
    "sf-mono": "settings.sfMono",
    menlo: "settings.menlo",
    consolas: "settings.consolas",
    "dejavu-mono": "settings.dejavuMono",
    "courier-new": "settings.courierNew",
    "ibm-plex-mono": "settings.ibmPlexMono",
    iosevka: "settings.iosevka",
    monaco: "settings.monaco",
    custom: "settings.custom",
};

function handleLocaleChange(l: string) {
    settings.setLocale(l);
    locale.value = l;
}
</script>

<style scoped>
.settings-dialog {
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 14px;
    width: 600px;
    height: 840px;
    max-height: 80vh;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    box-shadow: 0 24px 64px var(--shadow);
    animation: dialog-in 0.2s ease-out;
}

@keyframes dialog-in {
    from {
        opacity: 0;
        transform: scale(0.95) translateY(-10px);
    }
    to {
        opacity: 1;
        transform: scale(1) translateY(0);
    }
}

/* ── Header ── */
.settings-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 20px;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
}
.settings-header h2 {
    font-size: 16px;
    font-weight: 600;
    margin: 0;
}
.close-btn {
    width: 32px;
    height: 32px;
    border-radius: 8px;
}

/* ── Layout ── */
.settings-layout {
    display: flex;
    flex: 1;
    min-height: 0;
}

/* ── Tabs ── */
.settings-tabs {
    width: 130px;
    flex-shrink: 0;
    border-right: 1px solid var(--border);
    padding: 8px;
    display: flex;
    flex-direction: column;
    gap: 2px;
}
.settings-tab {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    border-radius: 6px;
    border: none;
    background: none;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 13px;
    text-align: left;
    transition: all 0.12s;
}
.settings-tab:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
}
.settings-tab.active {
    background: var(--accent);
    color: #fff;
}
.tab-icon {
    width: 16px;
    height: 16px;
    display: flex;
    align-items: center;
    flex-shrink: 0;
}
.tab-icon :deep(svg) {
    width: 16px;
    height: 16px;
}
.tab-label {
    white-space: nowrap;
}

/* ── Content ── */
.settings-content {
    flex: 1;
    overflow-y: auto;
    padding: 20px 24px;
}
.tab-panel {
    display: flex;
    flex-direction: column;
    gap: 20px;
}
.setting-group {
    display: flex;
    flex-direction: column;
    gap: 10px;
}
.group-title {
    font-size: 12px;
    font-weight: 600;
    text-transform: uppercase;
    color: var(--text-muted);
    letter-spacing: 0.5px;
}

/* ── Theme grid ── */
.theme-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
}

.theme-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    padding: 10px 8px;
    border: 2px solid var(--border);
    border-radius: 8px;
    background: var(--bg-primary);
    cursor: pointer;
    transition: all 0.15s;
}

.theme-card:hover {
    border-color: var(--text-muted);
}

.theme-card.active {
    border-color: var(--accent);
    background: var(--bg-hover);
}

.theme-bars {
    width: 100%;
    height: 28px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    padding: 4px 6px;
    gap: 4px;
}

.tb-bar {
    width: 20%;
    height: 100%;
    border-radius: 2px;
    opacity: 0.7;
}

.tb-spacer {
    flex: 1;
}

.tb-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
}

.theme-label {
    font-size: 13px;
    font-weight: 500;
}

/* ── Font size ── */
.font-setting-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
}
.font-badge {
    padding: 2px 8px;
    border-radius: 12px;
    background: var(--accent);
    color: var(--bg-primary);
    font-size: 11px;
    font-weight: 700;
}
.font-slider-container {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 6px 4px;
}
.font-slider-label {
    font-size: 11px;
    color: var(--text-muted);
    font-weight: 600;
}
.font-slider-range {
    flex: 1;
    -webkit-appearance: none;
    appearance: none;
    height: 6px;
    border-radius: 3px;
    background: var(--border);
    outline: none;
    cursor: pointer;
}
.font-slider-range::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: var(--accent);
    border: 2px solid var(--bg-primary);
    box-shadow: 0 1px 4px rgba(0,0,0,0.3);
    cursor: grab;
    transition: transform 0.1s, box-shadow 0.1s;
}
.font-slider-range::-webkit-slider-thumb:active {
    cursor: grabbing;
    transform: scale(1.15);
}
.font-presets {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-top: 4px;
}
.font-preset-chip {
    padding: 3px 8px;
    border-radius: 4px;
    border: 1px solid var(--border);
    background: var(--bg-secondary);
    color: var(--text-secondary);
    font-size: 11px;
    cursor: pointer;
    transition: all 0.15s;
}
.font-preset-chip:hover {
    border-color: var(--accent);
    color: var(--text-primary);
}
.font-preset-chip.active {
    background: var(--accent);
    color: var(--bg-primary);
    border-color: var(--accent);
    font-weight: 600;
}
.font-preview-box {
    margin-top: 10px;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: var(--bg-primary);
    overflow: hidden;
    box-shadow: inset 0 1px 3px rgba(0,0,0,0.1);
}
.font-preview-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 12px;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border);
}
.font-preview-badge {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
}
.font-preview-current {
    font-size: 11px;
    color: var(--text-muted);
}
.font-preview-list {
    display: flex;
    flex-direction: column;
}
.preview-row {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 0 12px;
    min-height: var(--row-height, 34px);
    border-bottom: 1px solid var(--border);
    font-size: var(--font-size-base, 13px);
    transition: min-height 0.1s ease, font-size 0.1s ease;
}
.preview-row:last-child {
    border-bottom: none;
}
.preview-icon {
    flex-shrink: 0;
}
.preview-name {
    flex: 2;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--text-primary);
}
.preview-type {
    flex: 1;
    font-size: var(--font-size-sm, 11px);
    color: var(--text-muted);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}
.preview-date {
    flex: 1.2;
    font-size: var(--font-size-sm, 11px);
    color: var(--text-muted);
    white-space: nowrap;
}
.preview-size {
    width: 60px;
    text-align: right;
    font-size: var(--font-size-sm, 11px);
    color: var(--text-muted);
    white-space: nowrap;
}
.font-options {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    gap: 10px;
}

.font-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    padding: 14px 12px;
    border: 2px solid var(--border);
    border-radius: 10px;
    background: var(--bg-primary);
    cursor: pointer;
    transition: all 0.15s;
}

.font-card:hover {
    border-color: var(--text-muted);
}

.font-card.active {
    border-color: var(--accent);
    background: var(--bg-hover);
}

.font-sample {
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1;
}

.font-label {
    font-size: 12px;
    color: var(--text-muted);
}

/* ── Language ── */
.lang-options {
    display: flex;
    flex-direction: column;
    gap: 6px;
}

.lang-card {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 14px;
    border: 1.5px solid var(--border);
    border-radius: 10px;
    background: var(--bg-primary);
    cursor: pointer;
    transition: all 0.15s;
}

.lang-card:hover {
    border-color: var(--text-muted);
}

.lang-card.active {
    border-color: var(--accent);
    background: var(--bg-hover);
}

.term-select-wrapper {
    position: relative;
}

.term-select-trigger {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    height: 36px;
    padding: 0 12px;
    background: var(--input-bg);
    border: 1px solid var(--border);
    border-radius: 8px;
    color: var(--text-primary);
    font-size: var(--font-size-base);
    cursor: pointer;
    text-align: left;
    transition: border-color 0.15s;
}
.term-select-trigger:hover,
.term-select-trigger.open {
    border-color: var(--accent);
}
.term-select-icon {
    display: flex;
    align-items: center;
    opacity: 0.6;
    flex-shrink: 0;
}
.term-select-label {
    flex: 1;
}
.term-select-arrow {
    flex-shrink: 0;
    color: var(--text-muted);
    transition: transform 0.2s;
}
.term-select-trigger.open .term-select-arrow {
    transform: rotate(180deg);
}
.term-select-dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    margin-top: 4px;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 4px;
    z-index: 10;
    box-shadow: 0 8px 32px var(--shadow);
}
.term-select-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 8px 10px;
    border: none;
    border-radius: 6px;
    background: none;
    color: var(--text-primary);
    font-size: var(--font-size-base);
    cursor: pointer;
    text-align: left;
    transition: background 0.1s;
}
.term-select-item:hover {
    background: var(--bg-hover);
}
.term-select-item.active {
    color: var(--accent);
}
.term-select-item-icon {
    display: flex;
    align-items: center;
    opacity: 0.5;
    flex-shrink: 0;
}
.term-select-item.active .term-select-item-icon {
    opacity: 1;
    color: var(--accent);
}
.term-select-item-label {
    flex: 1;
}
.term-select-item-check {
    flex-shrink: 0;
}

.lang-options {
    display: flex;
    flex-direction: column;
    gap: 6px;
}
.lang-flag {
    font-size: 24px;
}

.lang-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 1px;
}

.lang-name {
    font-size: 14px;
    font-weight: 500;
}
.lang-native {
    font-size: 12px;
    color: var(--text-muted);
}

.check-icon {
    width: 20px;
    height: 20px;
    flex-shrink: 0;
}

/* ── About ── */
.about-hero {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 24px 16px 20px;
}

.about-icon {
    width: 72px;
    height: 72px;
    border-radius: 18px;
    margin-bottom: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.about-name {
    font-size: 18px;
    font-weight: 700;
}

.about-version {
    font-size: 12px;
    color: var(--text-muted);
    font-family: monospace;
}

.about-desc {
    font-size: 12px;
    color: var(--text-secondary);
    text-align: center;
    line-height: 1.5;
    max-width: 280px;
    margin-top: 6px;
}

.about-pillars {
    display: flex;
    justify-content: center;
    gap: 8px;
    flex-wrap: wrap;
    margin: 16px 0 20px;
}

.pillar-chip {
    cursor: default;
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    padding: 5px 14px;
    background: var(--bg-primary);
    border: 1px solid var(--border);
    border-radius: 16px;
    display: inline-flex;
    align-items: center;
    gap: 5px;
}

.about-update-btn {
    width: calc(100% - 32px);
    margin: 0 auto;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 10px;
    font-size: var(--font-size-base);
    font-weight: 600;
    color: #1e1e2e;
    background: var(--accent);
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: opacity 0.15s, transform 0.1s;
}
.about-update-btn:hover:not(:disabled) {
    opacity: 0.9;
    transform: translateY(-1px);
}
.about-update-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    transform: none;
}

.about-links {
    display: flex;
    gap: 8px;
    margin: 0 16px;
}

.about-link-btn {
    flex: 1;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 7px 12px;
    font-size: var(--font-size-sm);
    color: var(--text-muted);
    background: var(--bg-primary);
    border: 1px solid var(--border);
    border-radius: 6px;
    cursor: pointer;
    transition: color 0.15s, border-color 0.15s;
}
.about-link-btn:hover {
    color: var(--text-primary);
    border-color: var(--text-muted);
}

.about-footer {
    text-align: center;
    font-size: 11px;
    color: var(--text-muted);
    padding: 16px 0 8px;
    opacity: 0.5;
}

@keyframes spin {
    to { transform: rotate(360deg); }
}
.spinner {
    animation: spin 0.8s linear infinite;
}

.icon-preview {
    width: 100%;
    height: 48px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 6px;
}
.fluent-preview {
    background: var(--bg-primary);
    border: 1px solid var(--border);
}
.material-preview {
    background: var(--bg-primary);
    border: 1px solid var(--border);
}
.ip-file {
    width: 22px;
    height: 26px;
    border-radius: 3px 3px 0 0;
    border: 1.5px solid;
    border-bottom: 3px solid;
    display: flex;
    align-items: flex-end;
    justify-content: center;
    font-size: 7px;
    font-weight: 700;
    padding-bottom: 1px;
}
.ip-blue {
    border-color: #4a6a9a;
    color: #4a6a9a;
}
.ip-green {
    border-color: #2d7d46;
    color: #2d7d46;
}
.ip-folder {
    width: 22px;
    height: 18px;
    border-radius: 3px;
    background: #f6c23a;
}
.mp-file,
.mp-folder {
    width: 24px;
    height: 24px;
    border-radius: 5px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #fff;
    font-size: 8px;
    font-weight: 700;
}

/* ── Checkbox ── */
.switch-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 12px;
    border-radius: 6px;
    cursor: pointer;
    transition: background 0.15s;
    gap: 16px;
}
.switch-row:hover {
    background: var(--bg-hover);
}
.switch-text {
    flex: 1;
    min-width: 0;
}
.switch-text span {
    display: block;
    color: var(--text-primary);
    font-size: 13px;
    font-weight: 500;
}
.switch-desc {
    color: var(--text-muted);
    font-size: 11px;
    margin-top: 2px;
}
.switch-track {
    width: 40px;
    height: 22px;
    border-radius: 11px;
    background: var(--bg-hover);
    flex-shrink: 0;
    position: relative;
    transition: background 0.2s;
    border: 1px solid var(--border);
}
.switch-track.active {
    background: var(--accent);
    border-color: var(--accent);
}
.switch-thumb {
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: #fff;
    position: absolute;
    top: 2px;
    left: 2px;
    transition: transform 0.2s;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}
.switch-track.active .switch-thumb {
    transform: translateX(18px);
}
</style>
