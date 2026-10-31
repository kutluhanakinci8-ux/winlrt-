(function () {
  "use strict";

  function t(key) {
    return window.WinlrtSite && WinlrtSite.t ? WinlrtSite.t(key) : key;
  }

  function pill(ok) {
    var passLabel = t("page.signing.gate.pass");
    var watchLabel = t("page.signing.gate.watch");
    var cls =
      ok === true
        ? "signing-pill signing-pill--ok"
        : ok === false
          ? "signing-pill signing-pill--warn"
          : "signing-pill";
    var mark = ok === true ? "✓" : ok === false ? "…" : "—";
    var label = ok === true ? passLabel : watchLabel;
    return '<span class="' + cls + '">' + mark + " " + label + "</span>";
  }

  function gateCard(title, pass, note) {
    return (
      '<article class="card signing-gate-card"><h3>' +
      title +
      '</h3><p class="meta">' +
      (note || "") +
      "</p>" +
      pill(pass) +
      "</article>"
    );
  }

  function loadSigningGates() {
    var metaEl = document.getElementById("signing-gate-meta");
    if (!metaEl) return;

    fetch("/benchmark/samples/lertiva-ev-ov-signing-gate.json", { cache: "no-store" })
      .then(function (r) {
        return r.json();
      })
      .then(function (ev) {
        var prod = ev.gates && ev.gates.ev_ov_pfx_production;
        if (prod && prod.pass === false) {
          var banner = document.getElementById("signing-ev-banner");
          if (!banner) return;
          banner.hidden = false;
          banner.innerHTML =
            "<p><strong>" +
            t("page.signing.evBanner.title") +
            ":</strong> " +
            (ev.disclaimer || "") +
            " · <code>" +
            (ev.certificate_tier || "") +
            "</code></p>";
        }
      })
      .catch(function () {});

    fetch("/benchmark/samples/lertiva-windows-signing-gate.json", { cache: "no-store" })
      .then(function (r) {
        return r.json();
      })
      .then(function (data) {
        var g = data.gates || {};
        var grid = document.getElementById("signing-gate-grid");
        if (!grid) return;
        grid.innerHTML =
          gateCard(
            t("page.signing.gate.zip"),
            g.ci_zip_signed_build && g.ci_zip_signed_build.pass,
            g.ci_zip_signed_build && g.ci_zip_signed_build.workflow
          ) +
          gateCard(
            t("page.signing.gate.arc"),
            g.ci_arc_signed_build && g.ci_arc_signed_build.pass,
            g.ci_arc_signed_build && g.ci_arc_signed_build.workflow
          ) +
          gateCard(
            t("page.signing.gate.pipeline"),
            g.signing_pipeline && g.signing_pipeline.pass,
            g.signing_pipeline && g.signing_pipeline.note
          ) +
          gateCard(
            t("page.signing.gate.png"),
            g.vitrin_smartscreen_real && g.vitrin_smartscreen_real.pass,
            g.vitrin_smartscreen_real && g.vitrin_smartscreen_real.note
          );
        metaEl.textContent = (data.disclaimer || "") + " · " + (data.generated_at || "");
      })
      .catch(function () {
        metaEl.textContent = t("page.signing.status.loadError");
      });
  }

  document.addEventListener("DOMContentLoaded", function () {
    loadSigningGates();
    document.querySelectorAll("[data-lang-btn]").forEach(function (btn) {
      btn.addEventListener("click", function () {
        setTimeout(loadSigningGates, 0);
      });
    });
  });
})();
