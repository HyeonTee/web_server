(function () {
  // ---------- Theme (must run before paint to avoid flash) ----------
  const root = document.documentElement;
  const stored = localStorage.getItem('theme');
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  const initial = stored || (prefersDark ? 'dark' : 'light');
  root.setAttribute('data-theme', initial);

  function applyTheme(theme) {
    root.setAttribute('data-theme', theme);
    localStorage.setItem('theme', theme);
    const btn = document.getElementById('theme-toggle');
    if (btn) btn.textContent = theme === 'dark' ? '☀' : '☾';
  }

  // ---------- Wire up after DOM ready ----------
  document.addEventListener('DOMContentLoaded', function () {
    // Toggle button
    const btn = document.getElementById('theme-toggle');
    if (btn) {
      btn.textContent = root.getAttribute('data-theme') === 'dark' ? '☀' : '☾';
      btn.addEventListener('click', function () {
        const current = root.getAttribute('data-theme');
        applyTheme(current === 'dark' ? 'light' : 'dark');
      });
    }

    // Nav active state based on current path
    const path = window.location.pathname;
    document.querySelectorAll('.nav-link').forEach(function (link) {
      if (link.getAttribute('href') === path) link.classList.add('active');
    });

    // Current YYYY.MM for bio "current" row
    const dateEl = document.getElementById('current-date');
    if (dateEl) {
      const now = new Date();
      const y = now.getFullYear();
      const m = String(now.getMonth() + 1).padStart(2, '0');
      dateEl.textContent = `${y}.${m}`;
    }

    // Typing animation for home greeting
    const target = document.getElementById('typed-greeting');
    if (target) {
      const text = target.getAttribute('data-text') || '';
      target.textContent = '';
      const cursor = document.createElement('span');
      cursor.className = 'cursor';
      target.after(cursor);

      let i = 0;
      const interval = setInterval(function () {
        if (i < text.length) {
          target.textContent += text.charAt(i);
          i++;
        } else {
          clearInterval(interval);
        }
      }, 90);
    }
  });
})();
