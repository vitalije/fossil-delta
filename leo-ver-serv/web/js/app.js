(function() {
  var $el, $q, $qall, EVENTS, _visible_nodes, appstate, connect_file_selection, connect_node_revisions, connect_outline_revisions, connect_tree, docdispatch, drawTree, eldispatch, get_body, get_body_rev, get_leo_files, get_outline_rev, get_rev_count, install_editor, partition, rpartition, select_leo_file, set_outline, start_app, update_leo_files, visible_nodes, wait;

  $el = function(_id) {
    return document.getElementById(_id);
  };

  $q = function(q) {
    return document.querySelector(q);
  };

  $qall = function(q) {
    return document.querySelectorAll(q);
  };

  wait = function(t) {
    return new Promise(function(res, rej) {
      return setTimeout(res, t);
    });
  };

  partition = function(s, sep) {
    var i;
    i = s.indexOf(sep);
    if (i < 0) {
      return [s, '', ''];
    }
    return [s.slice(0, i), sep, s.slice(i + sep.length)];
  };

  rpartition = function(s, sep) {
    var i;
    i = s.lastIndexOf(sep);
    if (i < 0) {
      return [s, '', ''];
    }
    return [s.slice(0, i), sep, s.slice(i + sep.length)];
  };

  EVENTS = {
    GNX_SELECTED: 'gnxselected',
    LEO_FILE_SELECTED: 'leofileselected',
    LEOFILES_CHANGED: 'leofileschanged',
    NODE_RANGE_CHANGED: 'noderangechanged',
    NODE_REV_CHANGED: 'noderevchanged',
    OUTLINE_CHANGED: 'outlinechanged',
    RANGE_CHANGED: 'rangechanged',
    REV_CHANGED: 'revchanged',
    TOPINDEX_CHANGED: 'topindexchanged'
  };

  docdispatch = function(kind, data) {
    var e;
    e = new CustomEvent(kind, {
      detail: data
    });
    return document.dispatchEvent(e);
  };

  eldispatch = function(el, kind, data) {
    return el.dispatchEvent(new CustomEvent(kind, {
      detail: data
    }));
  };

  appstate = {};

  (function() {
    var ss;
    ss = {
      fi: 0,
      leoFiles: [],
      range: {
        tmin: '0',
        tmax: '0',
        n: 0,
        i: 0
      },
      nodeRange: {
        tmin: '0',
        tmax: '0',
        n: 0,
        i: 0
      },
      outline: [],
      topIndex: 0,
      currentIndex: -1
    };
    return Object.defineProperties(appstate, {
      currentIndex: {
        get: function() {
          return ss.currentIndex;
        },
        set: function(v) {
          var gnx, ref;
          ss.currentIndex = v;
          gnx = (ref = ss.outline[v]) != null ? ref.gnx : void 0;
          if (gnx) {
            return docdispatch(EVENTS.GNX_SELECTED, gnx);
          }
        }
      },
      currentGnx: {
        get: function() {
          var i, ref;
          i = ss.currentIndex;
          return (ref = ss.outline[i]) != null ? ref.gnx : void 0;
        }
      },
      range: {
        get: function() {
          return {...ss.range};
        },
        set: function(v) {
          ss.range.tmin = v.tmin;
          ss.range.tmax = v.tmax;
          ss.range.n = v.n;
          if (v.n < ss.range.i) {
            ss.range.i = v.n;
          }
          v.i = ss.range.i;
          return docdispatch(EVENTS.RANGE_CHANGED, v);
        }
      },
      nodeRange: {
        get: function() {
          return {...ss.nodeRange};
        },
        set: function(v) {
          ss.nodeRange.tmin = v.tmin;
          ss.nodeRange.tmax = v.tmax;
          ss.nodeRange.n = v.n;
          if (v.n < ss.nodeRange.i) {
            ss.nodeRange.i = v.n;
          }
          v.i = ss.nodeRange.i;
          return docdispatch(EVENTS.NODE_RANGE_CHANGED, v);
        }
      },
      rev: {
        get: function() {
          return ss.range.i;
        },
        set: function(x) {
          ss.range.i = Math.min(ss.range.n, x);
          return docdispatch(EVENTS.REV_CHANGED, ss.range.i);
        }
      },
      nodeRev: {
        get: function() {
          return ss.nodeRange.i;
        },
        set: function(x) {
          ss.nodeRange.i = Math.min(ss.nodeRange.n, x);
          return docdispatch(EVENTS.NODE_REV_CHANGED, ss.nodeRange.i);
        }
      },
      leoFiles: {
        get: function() {
          return ss.leoFiles;
        },
        set: function(v) {
          ss.leoFiles.splice(0, ss.leoFiles.length, ...v);
          return docdispatch(EVENTS.LEOFILES_CHANGED, v);
        }
      },
      outline: {
        get: function() {
          return ss.outline;
        },
        set: function(v) {
          ss.outline.splice(0, ss.outline.length, ...v);
          if (ss.topIndex > v.length - 30) {
            ss.topIndex = Math.max(0, v.length - 30);
          }
          return docdispatch(EVENTS.OUTLINE_CHANGED, v);
        }
      },
      topIndex: {
        get: function() {
          return ss.topIndex;
        },
        set: function(v) {
          ss.topIndex = v;
          return docdispatch(EVENTS.TOPINDEX_CHANGED, v);
        }
      },
      currentFile: {
        get: function() {
          return ss.leoFiles[ss.fi];
        },
        set: function(v) {
          ss.fi = ss.leoFiles.indexOf(v);
          return docdispatch(EVENTS.LEO_FILE_SELECTED, v);
        }
      },
      currentFileIndex: {
        get: function() {
          return ss.fi;
        },
        set: function(v) {
          ss.fi = v;
          return docdispatch(EVENTS.LEO_FILE_SELECTED, ss.leoFiles[v]);
        }
      },
      topNode: {
        get: function() {
          return ss.outline[ss.topIndex];
        }
      }
    });
  })();

  window._state = appstate;

  get_rev_count = function(gnx) {
    var fname, rq;
    fname = appstate.currentFile;
    rq = {
      method: 'POST',
      body: `${fname}\n${gnx}`
    };
    return fetch('/node-rev-count', rq).then(function(x) {
      return x.json();
    }).then(function(x) {
      if (gnx === '__outline__') {
        return appstate.range = x;
      } else {
        return appstate.nodeRange = x;
      }
    });
  };

  get_leo_files = function() {
    return fetch('/leo-files').then(function(x) {
      return x.text();
    }).then(function(x) {
      return appstate.leoFiles = x.split('\n');
    });
  };

  partition = function(s, t) {
    var i;
    i = s.indexOf(t);
    if (i > -1) {
      return [s.slice(0, i), t, s.slice(i + t.length)];
    } else {
      return [s, '', ''];
    }
  };

  set_outline = function(data) {
    var lines, outline;
    outline = appstate.outline.splice(0);
    lines = data.split('\n');
    appstate.outlineTime = lines.shift();
    lines.shift();
    outline.splice(lines.length);
    lines.forEach(function(line, i) {
      var gnx, h, lev, node, rest, sep;
      node = outline[i];
      if (node == null) {
        node = {
          gnx: '',
          lev: 0,
          exp: false,
          h: '',
          old: false
        };
      }
      [lev, sep, rest] = partition(line, ' ');
      [gnx, sep, h] = partition(rest, ' ');
      node.gnx = gnx;
      node.lev = parseInt(lev) - 1;
      node.h = h;
      if (!node.old) {
        node.old = true;
        node.exp = node.lev < 3;
      }
      return outline[i] = node;
    });
    _visible_nodes._ver++;
    return appstate.outline = outline;
  };

  get_outline_rev = function(rev) {
    var body, fname;
    fname = appstate.currentFile;
    body = fname + '\n__outline__ ' + rev;
    return fetch('/node-rev', {
      method: 'POST',
      body: body
    }).then(function(x) {
      return x.text();
    }).then(set_outline);
  };

  drawTree = function() {
    var HR, NR, canv, ctx, h, i, inv, selIndex, vnodes, w, y;
    canv = $el('tree');
    w = canv.width;
    h = canv.height;
    ctx = canv.getContext('2d');
    ctx.fillStyle = '#336699';
    ctx.fillRect(0, 0, w, h);
    ctx.font = '20px sans';
    HR = 24;
    NR = Math.floor(h / HR);
    i = appstate.topIndex;
    vnodes = visible_nodes().slice(i, i + NR);
    selIndex = appstate.currentIndex;
    inv = function() {
      ctx.fillStyle = '#ffffce';
      ctx.fillRect(0, y - 20, w, HR);
      return ctx.fillStyle = '#336699';
    };
    ctx.fillStyle = '#ffffce';
    y = HR;
    vnodes.forEach(function(vn, j) {
      if (selIndex === vn.i) {
        inv();
      }
      if (vn.pref) {
        ctx.fillText(vn.pref, vn.x - 20, y);
      }
      ctx.fillText(vn.h, vn.x, y);
      if (selIndex === vn.i) {
        ctx.fillStyle = '#ffffce';
      }
      return y += HR;
    });
    return ctx = null;
  };

  _visible_nodes = {
    _ver: 0,
    _last_ver: -1,
    nodes: []
  };

  visible_nodes = function() {
    var N, hasChildren, i, n, n2, nodes, outline, pref;
    if (_visible_nodes._last_ver === _visible_nodes._ver) {
      return _visible_nodes.nodes;
    }
    _visible_nodes._last_ver = _visible_nodes._ver;
    nodes = [];
    outline = appstate.outline;
    i = 0;
    N = outline.length;
    while (i < N) {
      n = outline[i];
      n2 = outline[i + 1];
      hasChildren = n2 && n2.lev > n.lev;
      if (hasChildren) {
        pref = n.exp ? '▼' : '▶';
      } else {
        pref = false;
      }
      nodes.push({
        i: i,
        gnx: n.gnx,
        h: n.h,
        pref: pref,
        x: n.lev * 36 + 28
      });
      i += 1;
      if (hasChildren && !n.exp) {
        while (n2 && n2.lev > n.lev) {
          i += 1;
          n2 = outline[i];
        }
      }
    }
    _visible_nodes.nodes = nodes;
    return nodes;
  };

  get_body = function(e) {
    var gnx, rq;
    gnx = e.detail;
    rq = {
      method: 'POST',
      body: appstate.currentFile + '\n' + gnx + ' ' + appstate.outlineTime
    };
    return fetch('/node-at', rq).then(function(x) {
      return x.text();
    }).then(function(x) {
      _cm.setValue(x);
      return get_rev_count(gnx);
    });
  };

  get_body_rev = function(gnx, rev) {
    var rq;
    rq = {
      method: 'POST',
      body: appstate.currentFile + '\n' + gnx + ' ' + appstate.nodeRev
    };
    return fetch('/node-rev', rq).then(function(x) {
      return x.text();
    }).then(function(x) {
      x = partition(x, '\n')[2];
      return _cm.setValue(x);
    });
  };

  connect_outline_revisions = function() {
    var orevinp, orevspan;
    orevinp = $el('outline-revs');
    orevspan = $el('orev');
    Kefir.fromEvents(orevinp, 'input').onValue(function() {
      var i, n, v;
      v = parseInt(orevinp.value);
      n = appstate.range.n;
      if (v === n) {
        orevspan.innerText = 'latest';
      } else {
        orevspan.innerText = (v - n).toString();
      }
      i = n - v;
      return appstate.rev = i;
    });
    document.addEventListener(EVENTS.RANGE_CHANGED, function(e) {
      var r;
      r = e.detail;
      orevinp.max = r.n;
      orevinp.value = r.n - r.i;
      return appstate.rev = r.i;
    });
    return document.addEventListener(EVENTS.REV_CHANGED, function(ev) {
      return get_outline_rev(ev.detail);
    });
  };

  connect_node_revisions = function() {
    var nrevinp, nrevspan;
    nrevinp = $el('node-revs');
    nrevspan = $el('nrev');
    Kefir.fromEvents(nrevinp, 'input').onValue(function() {
      var i, n, r, v;
      v = parseInt(nrevinp.value);
      r = appstate.nodeRange;
      n = r.n;
      if (v === n) {
        nrevspan.innerText = 'latest';
      } else {
        nrevspan.innerText = (v - n).toString();
      }
      i = n - v;
      return appstate.nodeRev = i;
    });
    document.addEventListener(EVENTS.NODE_RANGE_CHANGED, function(e) {
      var r;
      r = e.detail;
      nrevinp.max = r.n;
      nrevinp.value = r.n - r.i;
      return appstate.nodeRev = r.i;
    });
    return document.addEventListener(EVENTS.NODE_REV_CHANGED, function(ev) {
      var gnx;
      gnx = appstate.currentGnx;
      return get_body_rev(gnx, ev.detail);
    });
  };

  connect_tree = function(h) {
    var canv, ctx;
    canv = $el('tree');
    canv.height = h;
    canv.width = 400;
    ctx = canv.getContext('2d');
    ctx.fillStyle = '#336699';
    ctx.fillRect(0, 0, 400, h);
    Kefir.fromEvents(canv, 'mousedown').onValue(function(e) {
      var HR, node, r, vn, vnodes, x, y;
      HR = 24;
      r = canv.getBoundingClientRect();
      x = e.x - r.left;
      y = e.y - r.top;
      vnodes = visible_nodes().slice(appstate.topIndex);
      vn = vnodes[Math.floor(y / HR)];
      if (!vn) {
        return;
      }
      if (x < vn.x && vn.pref) {
        node = appstate.outline[vn.i];
        node.exp = !node.exp;
        _visible_nodes._ver++;
      }
      return appstate.currentIndex = vn.i;
    });
    document.addEventListener(EVENTS.OUTLINE_CHANGED, drawTree);
    document.addEventListener(EVENTS.TOPINDEX_CHANGED, drawTree);
    document.addEventListener(EVENTS.TOPINDEX_CHANGED, drawTree);
    document.addEventListener(EVENTS.GNX_SELECTED, drawTree);
    return document.addEventListener(EVENTS.GNX_SELECTED, get_body);
  };

  select_leo_file = function() {
    console.log(appstate.currentFile, 'selected');
    if (!appstate.currentFile) {
      return;
    }
    return get_rev_count('__outline__').then(function() {
      _visible_nodes._ver++;
      appstate.currentIndex = Math.min(appstate.currentIndex, visible_nodes().length - 1);
      return appstate.rev = 0;
    });
  };

  update_leo_files = function() {
    var lfs, mkopt, results, s;
    s = $el('leo-files');
    mkopt = function(f, i) {
      var x;
      x = document.createElement('option');
      x.value = i;
      x.innerText = rpartition(f, '/')[2];
      return x;
    };
    lfs = appstate.leoFiles;
    lfs.forEach(function(f, i) {
      if (i >= s.options.length) {
        return s.options.add(mkopt(f, i));
      } else {
        s.options[i].value = i;
        return s.options[i].innerText = rpartition(f, '/')[2];
      }
    });
    results = [];
    while (s.options.length > lfs.length) {
      results.push(s.options.remove(lfs.length));
    }
    return results;
  };

  connect_file_selection = function() {
    var s;
    s = $el('leo-files');
    Kefir.fromEvents(s, 'change').onValue(function() {
      return appstate.currentFileIndex = s.selectedIndex;
    });
    document.addEventListener(EVENTS.LEO_FILE_SELECTED, select_leo_file);
    return document.addEventListener(EVENTS.LEOFILES_CHANGED, update_leo_files);
  };

  install_editor = function() {
    return window._cm = CodeMirror.fromTextArea($el('body'), {
      lineNumbers: true,
      theme: 'abcdef'
    });
  };

  start_app = function() {
    var h;
    h = window.innerHeight - $el('toolbar').getBoundingClientRect().height;
    $q('.app-flex').style.height = `${h}px`;
    install_editor();
    connect_file_selection();
    connect_tree(h);
    connect_outline_revisions();
    connect_node_revisions();
    get_leo_files().then(function() {
      return get_rev_count('__outline__');
    });
    return wait(20).then(function() {
      return appstate.rev = 0;
    });
  };

  window.addEventListener('load', start_app);

}).call(this);
