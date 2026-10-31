(function () {
  "use strict";

  /** Tek kaynak — tüm vitrin sayfalarında ortak alt bar. */
  function footerInnerHtml() {
    return (
      '<div class="container">' +
      '<div class="footer-grid">' +
      '<div class="footer-brand">' +
      '<a class="logo footer-logo" href="/">' +
      '<img class="logo-img" src="/assets/logo-mark.svg" width="40" height="40" alt="" />' +
      '<span>Lertiva</span></a>' +
      '<p class="footer-eyebrow" data-i18n="footer.eyebrow">Yerli arşiv yazılımı</p>' +
      '<p data-i18n="footer.tagline"></p>' +
      '<div class="footer-social" role="navigation" aria-label="Sosyal">' +
      socialLink(
        "github",
        "https://github.com/kutluhanakinci8-ux/winlrt-",
        "footer.social.github"
      ) +
      socialLink(
        "linkedin",
        "https://www.lerta.com.tr/",
        "footer.social.lerta"
      ) +
      socialLink(
        "x",
        "https://github.com/kutluhanakinci8-ux/winlrt-/issues",
        "footer.social.community"
      ) +
      socialLink(
        "youtube",
        "https://winlrt.com.tr/benchmark/",
        "footer.social.demos"
      ) +
      "</div></div>" +
      col(
        "footer.products",
        link("/downloads/#lertiva-arc", "footer.link.arc") +
          link("/downloads/#lertiva-zip", "footer.link.zip") +
          link("/downloads/", "footer.link.downloads") +
          link("/downloads/#lertiva-zip", "footer.link.macos") +
          link("/downloads/lertiva-arc-corporate-gpo.zip", "footer.link.gpoArc") +
          link("/downloads/lertiva-zip-corporate-gpo.zip", "footer.link.gpoZip")
      ) +
      col(
        "footer.features",
        link("/#cozumler", "footer.link.engine") +
          link("/pilot/", "footer.link.explorer") +
          link("/benchmark/corporate/", "footer.link.corporate") +
          link("/kvkk/", "footer.link.audit") +
          link("/benchmark/lerta/", "footer.link.lertaMeta") +
          link("/benchmark/", "footer.link.kpiBench")
      ) +
      col(
        "footer.quick",
        link("/", "footer.link.home") +
          link("/benchmark/", "footer.link.kpi") +
          link("/pilot/", "footer.link.pilot") +
          link("https://www.lerta.com.tr", "footer.link.lerta", true) +
          link("https://github.com/kutluhanakinci8-ux/winlrt-", "footer.link.github", true) +
          link("/iletisim/", "footer.link.contact") +
          link("/marka/", "footer.link.brand") +
          link(
            "https://github.com/kutluhanakinci8-ux/winlrt-/blob/main/docs/qa/POST_V3_WINLRTZIP_SIRA.md",
            "footer.link.roadmap",
            true
          )
      ) +
      col(
        "footer.legalCol",
        link("/kvkk/", "footer.link.kvkk") +
          link("/hakkimizda/", "footer.link.about") +
          link("/iletisim/", "footer.link.imprint") +
          link("/downloads/#imza", "footer.link.signing") +
          link("/kvkk/#cerez", "footer.link.cookies")
      ) +
      "</div>" +
      '<div class="footer-bottom">' +
      '<div class="footer-bottom-social footer-social footer-social--compact">' +
      socialLink(
        "github",
        "https://github.com/kutluhanakinci8-ux/winlrt-",
        "footer.social.github"
      ) +
      socialLink(
        "linkedin",
        "https://www.lerta.com.tr/",
        "footer.social.lerta"
      ) +
      socialLink(
        "x",
        "https://github.com/kutluhanakinci8-ux/winlrt-/issues",
        "footer.social.community"
      ) +
      socialLink(
        "youtube",
        "https://winlrt.com.tr/benchmark/",
        "footer.social.demos"
      ) +
      "</div>" +
      '<p class="footer-copy" data-i18n="footer.copy"></p>' +
      '<p class="footer-region" data-i18n="footer.region"></p>' +
      "</div></div>"
    );
  }

  function col(titleKey, linksHtml) {
    return (
      '<div class="footer-col">' +
      '<h4 data-i18n="' +
      titleKey +
      '"></h4>' +
      linksHtml +
      "</div>"
    );
  }

  function link(href, i18nKey, external) {
    var ext =
      external ||
      (href.indexOf("http") === 0 &&
        href.indexOf("https://winlrt.com.tr") !== 0);
    var rel = ext ? ' rel="noopener noreferrer"' : "";
    var extClass = ext ? " footer-ext" : "";
    return (
      '<a href="' +
      href +
      '" class="footer-link' +
      extClass +
      '"' +
      rel +
      ' data-i18n="' +
      i18nKey +
      '"></a>'
    );
  }

  function socialLink(id, href, labelKey) {
    return (
      '<a class="footer-social-btn" href="' +
      href +
      '" rel="noopener noreferrer" data-i18n-title="' +
      labelKey +
      '">' +
      socialIcon(id) +
      '<span class="visually-hidden" data-i18n="' +
      labelKey +
      '"></span></a>'
    );
  }

  function socialIcon(id) {
    var svg =
      '<svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor" aria-hidden="true">';
    if (id === "github") {
      return (
        svg +
        '<path d="M12 2C6.477 2 2 6.484 2 12.021c0 4.428 2.865 8.184 6.839 9.504.5.092.682-.217.682-.483 0-.237-.009-.868-.014-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.021C22 6.484 17.522 2 12 2z"/></svg>'
      );
    }
    if (id === "linkedin") {
      return (
        svg +
        '<path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 01-2.063-2.065 2.064 2.064 0 114.127 0 2.063 2.063 0 01-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/></svg>'
      );
    }
    if (id === "x") {
      return (
        svg +
        '<path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/></svg>'
      );
    }
    if (id === "youtube") {
      return (
        svg +
        '<path d="M23.498 6.186a3.016 3.016 0 00-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 00.502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 002.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 002.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z"/></svg>'
      );
    }
    return svg + "</svg>";
  }

  function mountSiteFooter() {
    var nodes = document.querySelectorAll("[data-site-footer]");
    if (!nodes.length) return;
    var html = footerInnerHtml();
    nodes.forEach(function (el) {
      el.innerHTML = html;
    });
  }

  window.mountSiteFooter = mountSiteFooter;
})();
