(function () {
  "use strict";

  function dropGroup(labelKey, items) {
    var links = items
      .map(function (it) {
        var ext = it.ext ? ' rel="noopener noreferrer"' : "";
        var cls = it.ext ? " footer-ext" : "";
        return (
          '<a class="nav-drop-link' +
          cls +
          '" href="' +
          it.href +
          '"' +
          ext +
          ' data-i18n="' +
          it.key +
          '"></a>'
        );
      })
      .join("");
    return (
      '<div class="nav-drop">' +
      '<button type="button" class="nav-drop-toggle" aria-expanded="false" data-i18n="' +
      labelKey +
      '"></button>' +
      '<div class="nav-drop-panel">' +
      links +
      "</div></div>"
    );
  }

  function headerHtml() {
    var products = dropGroup("nav.drop.products", [
      { href: "/urunler/lertiva-arc/", key: "nav.drop.arc" },
      { href: "/urunler/lertiva-zip/", key: "nav.drop.zip" },
      { href: "/urunler/", key: "nav.drop.allProducts" },
      { href: "/downloads/", key: "nav.download" },
      { href: "/downloads/lertiva-arc-corporate-gpo.zip", key: "nav.drop.gpoArc" },
      { href: "/downloads/lertiva-zip-corporate-gpo.zip", key: "nav.drop.gpoZip" },
    ]);
    var features = dropGroup("nav.drop.features", [
      { href: "/ozellikler/", key: "nav.drop.allFeatures" },
      { href: "/cozumler/", key: "nav.cozumler" },
      { href: "/guven/", key: "nav.guven" },
      { href: "/ozellikler/#motor", key: "nav.drop.engine" },
      { href: "/ozellikler/#guvenlik", key: "nav.drop.security" },
      { href: "/ozellikler/#explorer", key: "nav.drop.explorer" },
      { href: "/ozellikler/#lerta", key: "nav.drop.lerta" },
    ]);
    var resources = dropGroup("nav.drop.resources", [
      { href: "/kaynaklar/", key: "nav.drop.allResources" },
      { href: "/kaynaklar/#yol-haritasi", key: "nav.drop.roadmapShort" },
      { href: "/kaynaklar/#lerta", key: "nav.drop.lertaHub" },
      { href: "/benchmark/", key: "nav.kpi" },
      { href: "/pilot/", key: "nav.pilot" },
      { href: "/marka/", key: "nav.brand" },
      {
        href: "https://github.com/kutluhanakinci8-ux/winlrt-",
        key: "nav.drop.github",
        ext: true,
      },
    ]);
    var support = dropGroup("nav.drop.support", [
      { href: "/destek/", key: "nav.drop.allSupport" },
      { href: "/iletisim/", key: "nav.contact" },
      { href: "/kvkk/", key: "nav.kvkk" },
      { href: "/downloads/#imza", key: "nav.drop.signing" },
      { href: "/hakkimizda/", key: "nav.about" },
    ]);

    return (
      '<header class="site-header">' +
      '<div class="site-header-inner">' +
      '<a class="logo" href="/">' +
      '<img class="logo-img" src="/assets/logo-mark.svg?v=10" width="52" height="52" alt="" />' +
      "<span>Lertiva</span></a>" +
      '<button type="button" class="menu-toggle" aria-expanded="false" aria-controls="main-nav" data-i18n="nav.menu">Menü</button>' +
      '<div class="header-tools">' +
      '<button type="button" class="tool-btn" id="theme-toggle" aria-label="Tema">◐</button>' +
      '<div class="lang-switch" role="group" aria-label="Dil">' +
      '<button type="button" data-lang-btn="tr">TR</button>' +
      '<button type="button" data-lang-btn="en">EN</button>' +
      "</div></div>" +
      '<nav class="nav" id="main-nav" aria-label="Ana menü">' +
      products +
      features +
      resources +
      support +
      '<a class="nav-cta" href="/downloads/" data-i18n="nav.download">İndir</a>' +
      "</nav></div></header>"
    );
  }

  function mountSiteHeader() {
    document.querySelectorAll("[data-site-header]").forEach(function (el) {
      el.innerHTML = headerHtml();
    });
  }

  window.mountSiteHeader = mountSiteHeader;

  if (document.querySelector("[data-site-header]")) {
    mountSiteHeader();
  }
})();
