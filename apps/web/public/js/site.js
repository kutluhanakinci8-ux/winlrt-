(function () {
  "use strict";

  var STORAGE_THEME = "winlrt-theme";
  var STORAGE_LANG = "winlrt-lang";

  function dict(lang) {
    var pack = window.WINLRT_I18N || {};
    return pack[lang] || pack.tr || {};
  }

  function getLang() {
    var stored = localStorage.getItem(STORAGE_LANG);
    if (stored === "en" || stored === "tr") return stored;
    var nav = (navigator.language || "tr").toLowerCase();
    return nav.indexOf("en") === 0 ? "en" : "tr";
  }

  function getTheme() {
    var stored = localStorage.getItem(STORAGE_THEME);
    if (stored === "light" || stored === "dark") return stored;
    return "dark";
  }

  function t(key, lang) {
    var d = dict(lang || getLang());
    var val = d[key] != null ? d[key] : key;
    if (key === "footer.copy" && val.indexOf("{year}") !== -1) {
      val = val.replace("{year}", String(new Date().getFullYear()));
    }
    return val;
  }

  function applyI18n(lang) {
    lang = lang || getLang();
    document.documentElement.lang = lang === "en" ? "en" : "tr";
    document.querySelectorAll("[data-i18n]").forEach(function (el) {
      var key = el.getAttribute("data-i18n");
      var val = t(key, lang);
      if (el.tagName === "INPUT" || el.tagName === "TEXTAREA") {
        el.placeholder = val;
      } else {
        el.textContent = val;
      }
    });
    document.querySelectorAll("[data-i18n-html]").forEach(function (el) {
      el.innerHTML = t(el.getAttribute("data-i18n-html"), lang);
    });
    document.querySelectorAll("[data-i18n-title]").forEach(function (el) {
      el.setAttribute("title", t(el.getAttribute("data-i18n-title"), lang));
    });
    document.querySelectorAll("[data-i18n-alt]").forEach(function (el) {
      el.setAttribute("alt", t(el.getAttribute("data-i18n-alt"), lang));
    });
    var metaDesc = document.querySelector('meta[name="description"]');
    var metaKey =
      (document.body && document.body.dataset.metaDescKey) || "meta.description";
    if (metaDesc && t(metaKey, lang) && t(metaKey, lang) !== metaKey) {
      metaDesc.setAttribute("content", t(metaKey, lang));
    }
    if (document.title && document.body.dataset.pageTitleKey) {
      document.title = t(document.body.dataset.pageTitleKey, lang);
    } else if (document.body.dataset.page === "home") {
      document.title = t("meta.title", lang);
    }
    document.querySelectorAll("[data-lang-btn]").forEach(function (btn) {
      var active = btn.getAttribute("data-lang-btn") === lang;
      btn.classList.toggle("is-active", active);
      btn.setAttribute("aria-pressed", active ? "true" : "false");
    });
  }

  function applyTheme(theme) {
    document.documentElement.setAttribute("data-theme", theme);
    var toggle = document.getElementById("theme-toggle");
    if (toggle) {
      var label = theme === "light" ? t("theme.dark") : t("theme.light");
      toggle.setAttribute("aria-label", label);
      toggle.setAttribute("title", label);
      toggle.textContent = theme === "light" ? "☀" : "◐";
    }
  }

  function closeNavDrops() {
    document.querySelectorAll(".nav-drop-toggle").forEach(function (btn) {
      btn.setAttribute("aria-expanded", "false");
      var panel = btn.nextElementSibling;
      if (panel) panel.classList.remove("is-open");
    });
  }

  function initNavDropdowns() {
    document.querySelectorAll(".nav-drop-toggle").forEach(function (btn) {
      btn.addEventListener("click", function (e) {
        e.preventDefault();
        e.stopPropagation();
        var panel = btn.nextElementSibling;
        var wasOpen = btn.getAttribute("aria-expanded") === "true";
        closeNavDrops();
        if (!wasOpen && panel) {
          btn.setAttribute("aria-expanded", "true");
          panel.classList.add("is-open");
        }
      });
    });
    document.addEventListener("click", closeNavDrops);
  }

  function initMenu() {
    var btn = document.querySelector(".menu-toggle");
    var nav = document.getElementById("main-nav");
    if (!btn || !nav) return;
    btn.addEventListener("click", function () {
      var open = nav.classList.toggle("is-open");
      btn.setAttribute("aria-expanded", open ? "true" : "false");
      if (!open) closeNavDrops();
    });
  }

  function initTheme() {
    applyTheme(getTheme());
    var toggle = document.getElementById("theme-toggle");
    if (!toggle) return;
    toggle.addEventListener("click", function () {
      var next = getTheme() === "light" ? "dark" : "light";
      localStorage.setItem(STORAGE_THEME, next);
      applyTheme(next);
    });
  }

  function initLang() {
    applyI18n(getLang());
    document.querySelectorAll("[data-lang-btn]").forEach(function (btn) {
      btn.addEventListener("click", function () {
        var lang = btn.getAttribute("data-lang-btn");
        if (lang !== "tr" && lang !== "en") return;
        localStorage.setItem(STORAGE_LANG, lang);
        applyI18n(lang);
        applyTheme(getTheme());
        loadLegalEntity();
        window.dispatchEvent(new CustomEvent("winlrt:lang", { detail: { lang: lang } }));
      });
    });
  }

  function loadLegalEntity() {
    var block = document.getElementById("legal-entity-block");
    if (!block) return;
    fetch("/data/legal-entity.json", { cache: "no-store" })
      .then(function (r) {
        return r.ok ? r.json() : null;
      })
      .then(function (data) {
        if (!data) return;
        var lang = getLang();
        var name =
          lang === "en" ? data.controller_name_en : data.controller_name_tr;
        var trade = lang === "en" ? data.trade_name_en : data.trade_name_tr;
        var addr = lang === "en" ? data.address_en : data.address_tr;
        var note = lang === "en" ? data.note_en : data.note_tr;
        block.innerHTML =
          "<p><strong>" +
          t("legal.entity.controller") +
          ":</strong> " +
          name +
          "</p><p><strong>" +
          t("legal.entity.trade") +
          ":</strong> " +
          trade +
          "</p><p><strong>" +
          t("legal.entity.address") +
          ":</strong> " +
          addr +
          "</p>" +
          (data.mersis_no
            ? "<p><strong>MERSİS:</strong> " + data.mersis_no + "</p>"
            : "") +
          "<p class=\"meta\">" +
          t("legal.entity.updated") +
          ": " +
          (data.updated_at || "") +
          "</p><p class=\"meta\">" +
          note +
          "</p>";
      })
      .catch(function () {});
  }

  window.WinlrtSite = {
    t: t,
    getLang: getLang,
    applyI18n: applyI18n,
  };

  document.addEventListener("DOMContentLoaded", function () {
    if (typeof window.mountSiteHeader === "function") {
      window.mountSiteHeader();
    }
    if (typeof window.mountSiteFooter === "function") {
      window.mountSiteFooter();
    }
    initMenu();
    initNavDropdowns();
    initTheme();
    initLang();
    loadLegalEntity();
  });
})();
