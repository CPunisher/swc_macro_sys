"use strict";
(self["webpackChunk_mf_react_host"] = self["webpackChunk_mf_react_host"] || []).push([["src_bootstrap_jsx"], {
"./src/App.jsx": 
/*!*********************!*\
  !*** ./src/App.jsx ***!
  \*********************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__)
});
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! react */ "webpack/sharing/consume/default/react/react");
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var react_router_dom__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! react-router-dom */ "webpack/sharing/consume/default/react-router-dom/react-router-dom");
/* ESM import */var react_router_dom__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(react_router_dom__WEBPACK_IMPORTED_MODULE_1__);
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_2__);
/* ESM import */var _components_AppHeader__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./components/AppHeader */ "./src/components/AppHeader.jsx");
/* ESM import */var _components_AppSidebar__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ./components/AppSidebar */ "./src/components/AppSidebar.jsx");
/* ESM import */var _pages_Dashboard__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ./pages/Dashboard */ "./src/pages/Dashboard.jsx");
/* ESM import */var _pages_Analytics__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! ./pages/Analytics */ "./src/pages/Analytics.jsx");
/* ESM import */var _pages_Users__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! ./pages/Users */ "./src/pages/Users.jsx");
/* ESM import */var _pages_Settings__WEBPACK_IMPORTED_MODULE_8__ = __webpack_require__(/*! ./pages/Settings */ "./src/pages/Settings.jsx");
/* ESM import */var _pages_RemoteShowcase__WEBPACK_IMPORTED_MODULE_9__ = __webpack_require__(/*! ./pages/RemoteShowcase */ "./src/pages/RemoteShowcase.jsx");










var Content = antd__WEBPACK_IMPORTED_MODULE_2__.Layout.Content;
var LoadingFallback = function() {
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        className: "loading-container"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_2__.Spin, {
        size: "large",
        tip: "Loading..."
    }));
};
function App() {
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_router_dom__WEBPACK_IMPORTED_MODULE_1__.BrowserRouter, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_2__.Layout, {
        className: "app-layout"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_components_AppSidebar__WEBPACK_IMPORTED_MODULE_4__["default"], null), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_2__.Layout, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_components_AppHeader__WEBPACK_IMPORTED_MODULE_3__["default"], null), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Content, {
        className: "app-content"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react__WEBPACK_IMPORTED_MODULE_0__.Suspense, {
        fallback: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(LoadingFallback, null)
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_router_dom__WEBPACK_IMPORTED_MODULE_1__.Routes, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_router_dom__WEBPACK_IMPORTED_MODULE_1__.Route, {
        path: "/",
        element: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_router_dom__WEBPACK_IMPORTED_MODULE_1__.Navigate, {
            to: "/dashboard",
            replace: true
        })
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_router_dom__WEBPACK_IMPORTED_MODULE_1__.Route, {
        path: "/dashboard",
        element: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_pages_Dashboard__WEBPACK_IMPORTED_MODULE_5__["default"], null)
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_router_dom__WEBPACK_IMPORTED_MODULE_1__.Route, {
        path: "/analytics",
        element: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_pages_Analytics__WEBPACK_IMPORTED_MODULE_6__["default"], null)
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_router_dom__WEBPACK_IMPORTED_MODULE_1__.Route, {
        path: "/users",
        element: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_pages_Users__WEBPACK_IMPORTED_MODULE_7__["default"], null)
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_router_dom__WEBPACK_IMPORTED_MODULE_1__.Route, {
        path: "/remote-components",
        element: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_pages_RemoteShowcase__WEBPACK_IMPORTED_MODULE_9__["default"], null)
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_router_dom__WEBPACK_IMPORTED_MODULE_1__.Route, {
        path: "/settings",
        element: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_pages_Settings__WEBPACK_IMPORTED_MODULE_8__["default"], null)
    })))))));
}
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (App);


}),
"./src/bootstrap.jsx": 
/*!***************************!*\
  !*** ./src/bootstrap.jsx ***!
  \***************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! react */ "webpack/sharing/consume/default/react/react");
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var react_dom_client__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! react-dom/client */ "../../../node_modules/.pnpm/react-dom@18.3.1_react@18.3.1/node_modules/react-dom/client.js");
/* ESM import */var react_redux__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! react-redux */ "webpack/sharing/consume/default/react-redux/react-redux");
/* ESM import */var react_redux__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(react_redux__WEBPACK_IMPORTED_MODULE_2__);
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_3__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_3___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_3__);
/* ESM import */var _store__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ./store */ "./src/store/index.js");
/* ESM import */var _App__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ./App */ "./src/App.jsx");






var root = react_dom_client__WEBPACK_IMPORTED_MODULE_1__.createRoot(document.getElementById('root'));
root.render(/*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement((react__WEBPACK_IMPORTED_MODULE_0___default().StrictMode), null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_redux__WEBPACK_IMPORTED_MODULE_2__.Provider, {
    store: _store__WEBPACK_IMPORTED_MODULE_4__.store
}, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_3__.ConfigProvider, {
    theme: {
        token: {
            colorPrimary: '#1890ff',
            borderRadius: 6
        }
    }
}, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_App__WEBPACK_IMPORTED_MODULE_5__["default"], null)))));


}),
"./src/components/AppHeader.jsx": 
/*!**************************************!*\
  !*** ./src/components/AppHeader.jsx ***!
  \**************************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__)
});
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! react */ "webpack/sharing/consume/default/react/react");
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_1__);
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! @ant-design/icons */ "webpack/sharing/consume/default/@ant-design/icons/@ant-design/icons");
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__);



var Header = antd__WEBPACK_IMPORTED_MODULE_1__.Layout.Header;
var AppHeader = function() {
    var userMenuItems = [
        {
            key: 'profile',
            icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.UserOutlined, null),
            label: 'Profile'
        },
        {
            key: 'settings',
            icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.SettingOutlined, null),
            label: 'Settings'
        },
        {
            type: 'divider'
        },
        {
            key: 'logout',
            icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.LogoutOutlined, null),
            label: 'Logout'
        }
    ];
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Header, {
        className: "app-header",
        style: {
            padding: '0 24px'
        }
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        style: {
            flex: 1
        }
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("h2", {
        style: {
            margin: 0
        }
    }, "Module Federation React Demo")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        size: "large"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Badge, {
        count: 5,
        size: "small"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.BellOutlined, {
        style: {
            fontSize: 18,
            cursor: 'pointer'
        }
    })), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Dropdown, {
        menu: {
            items: userMenuItems
        },
        placement: "bottomRight"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Avatar, {
        style: {
            cursor: 'pointer',
            backgroundColor: '#1890ff'
        },
        icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.UserOutlined, null)
    }))));
};
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (AppHeader);


}),
"./src/components/AppSidebar.jsx": 
/*!***************************************!*\
  !*** ./src/components/AppSidebar.jsx ***!
  \***************************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__)
});
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! react */ "webpack/sharing/consume/default/react/react");
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_1__);
/* ESM import */var react_router_dom__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! react-router-dom */ "webpack/sharing/consume/default/react-router-dom/react-router-dom");
/* ESM import */var react_router_dom__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(react_router_dom__WEBPACK_IMPORTED_MODULE_2__);
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_3__ = /* #__PURE__ */ __webpack_require__(/*! @ant-design/icons */ "webpack/sharing/consume/default/@ant-design/icons/@ant-design/icons");
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_3___default = /*#__PURE__*/__webpack_require__.n(_ant_design_icons__WEBPACK_IMPORTED_MODULE_3__);




var Sider = antd__WEBPACK_IMPORTED_MODULE_1__.Layout.Sider;
var AppSidebar = function() {
    var navigate = (0,react_router_dom__WEBPACK_IMPORTED_MODULE_2__.useNavigate)();
    var location = (0,react_router_dom__WEBPACK_IMPORTED_MODULE_2__.useLocation)();
    var menuItems = [
        {
            key: '/dashboard',
            icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_3__.DashboardOutlined, null),
            label: 'Dashboard'
        },
        {
            key: '/analytics',
            icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_3__.BarChartOutlined, null),
            label: 'Analytics'
        },
        {
            key: '/users',
            icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_3__.UserOutlined, null),
            label: 'Users'
        },
        {
            key: '/remote-components',
            icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_3__.AppstoreOutlined, null),
            label: 'Remote Components'
        },
        {
            key: '/settings',
            icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_3__.SettingOutlined, null),
            label: 'Settings'
        }
    ];
    var handleMenuClick = function(param) {
        var key = param.key;
        navigate(key);
    };
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Sider, {
        width: 250,
        theme: "dark"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        style: {
            height: 64,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            borderBottom: '1px solid rgba(255, 255, 255, 0.1)'
        }
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("h3", {
        style: {
            color: '#fff',
            margin: 0
        }
    }, "MF React App")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Menu, {
        theme: "dark",
        mode: "inline",
        selectedKeys: [
            location.pathname
        ],
        items: menuItems,
        onClick: handleMenuClick
    }));
};
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (AppSidebar);


}),
"./src/pages/Analytics.jsx": 
/*!*********************************!*\
  !*** ./src/pages/Analytics.jsx ***!
  \*********************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__)
});
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! react */ "webpack/sharing/consume/default/react/react");
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_1__);
/* ESM import */var react_redux__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! react-redux */ "webpack/sharing/consume/default/react-redux/react-redux");
/* ESM import */var react_redux__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(react_redux__WEBPACK_IMPORTED_MODULE_2__);
/* ESM import */var _store_slices_analyticsSlice__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ../store/slices/analyticsSlice */ "./src/store/slices/analyticsSlice.js");
/* ESM import */var react_chartjs_2__WEBPACK_IMPORTED_MODULE_4__ = /* #__PURE__ */ __webpack_require__(/*! react-chartjs-2 */ "webpack/sharing/consume/default/react-chartjs-2/react-chartjs-2");
/* ESM import */var react_chartjs_2__WEBPACK_IMPORTED_MODULE_4___default = /*#__PURE__*/__webpack_require__.n(react_chartjs_2__WEBPACK_IMPORTED_MODULE_4__);
/* ESM import */var chart_js__WEBPACK_IMPORTED_MODULE_5__ = /* #__PURE__ */ __webpack_require__(/*! chart.js */ "webpack/sharing/consume/default/chart.js/chart.js");
/* ESM import */var chart_js__WEBPACK_IMPORTED_MODULE_5___default = /*#__PURE__*/__webpack_require__.n(chart_js__WEBPACK_IMPORTED_MODULE_5__);






chart_js__WEBPACK_IMPORTED_MODULE_5__.Chart.register(chart_js__WEBPACK_IMPORTED_MODULE_5__.CategoryScale, chart_js__WEBPACK_IMPORTED_MODULE_5__.LinearScale, chart_js__WEBPACK_IMPORTED_MODULE_5__.PointElement, chart_js__WEBPACK_IMPORTED_MODULE_5__.LineElement, chart_js__WEBPACK_IMPORTED_MODULE_5__.Title, chart_js__WEBPACK_IMPORTED_MODULE_5__.Tooltip, chart_js__WEBPACK_IMPORTED_MODULE_5__.Legend, chart_js__WEBPACK_IMPORTED_MODULE_5__.ArcElement);
var Title = antd__WEBPACK_IMPORTED_MODULE_1__.Typography.Title;
var Analytics = function() {
    var dispatch = (0,react_redux__WEBPACK_IMPORTED_MODULE_2__.useDispatch)();
    var _useSelector = (0,react_redux__WEBPACK_IMPORTED_MODULE_2__.useSelector)(function(state) {
        return state.analytics;
    }), data = _useSelector.data, loading = _useSelector.loading, timeRange = _useSelector.timeRange;
    (0,react__WEBPACK_IMPORTED_MODULE_0__.useEffect)(function() {
        dispatch((0,_store_slices_analyticsSlice__WEBPACK_IMPORTED_MODULE_3__.fetchAnalyticsData)());
    }, [
        dispatch,
        timeRange
    ]);
    var handleTimeRangeChange = function(value) {
        dispatch((0,_store_slices_analyticsSlice__WEBPACK_IMPORTED_MODULE_3__.setTimeRange)(value));
    };
    var chartOptions = {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
            legend: {
                position: 'top'
            }
        }
    };
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Row, {
        justify: "space-between",
        align: "middle",
        style: {
            marginBottom: 24
        }
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 2
    }, "Analytics")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select, {
        value: timeRange,
        onChange: handleTimeRangeChange,
        style: {
            width: 200
        },
        options: [
            {
                value: '7days',
                label: 'Last 7 days'
            },
            {
                value: '30days',
                label: 'Last 30 days'
            },
            {
                value: '6months',
                label: 'Last 6 months'
            },
            {
                value: '1year',
                label: 'Last year'
            }
        ]
    }))), loading ? /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        className: "loading-container"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Spin, {
        size: "large"
    })) : /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Row, {
        gutter: [
            16,
            16
        ]
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        lg: 12
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: "Revenue Trend"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        style: {
            height: 300
        }
    }, (data === null || data === void 0 ? void 0 : data.revenue) && /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_chartjs_2__WEBPACK_IMPORTED_MODULE_4__.Line, {
        data: data.revenue,
        options: chartOptions
    })))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        lg: 12
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: "User Growth"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        style: {
            height: 300
        }
    }, (data === null || data === void 0 ? void 0 : data.userGrowth) && /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_chartjs_2__WEBPACK_IMPORTED_MODULE_4__.Line, {
        data: data.userGrowth,
        options: chartOptions
    })))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        lg: 8
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: "Device Categories"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        style: {
            height: 300
        }
    }, (data === null || data === void 0 ? void 0 : data.categories) && /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_chartjs_2__WEBPACK_IMPORTED_MODULE_4__.Doughnut, {
        data: data.categories,
        options: chartOptions
    })))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        lg: 16
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: "Key Metrics"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        direction: "vertical",
        size: "large",
        style: {
            width: '100%'
        }
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Row, {
        gutter: 16
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        span: 8
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Statistic, {
        title: "Page Views",
        value: "1,234,567"
    })), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        span: 8
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Statistic, {
        title: "Unique Visitors",
        value: "456,789"
    })), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        span: 8
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Statistic, {
        title: "Avg. Session",
        value: "5m 32s"
    }))))))));
};
// Import Statistic component

/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (Analytics);


}),
"./src/pages/Dashboard.jsx": 
/*!*********************************!*\
  !*** ./src/pages/Dashboard.jsx ***!
  \*********************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__)
});
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! react */ "webpack/sharing/consume/default/react/react");
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_1__);
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! @ant-design/icons */ "webpack/sharing/consume/default/@ant-design/icons/@ant-design/icons");
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__);
/* ESM import */var react_redux__WEBPACK_IMPORTED_MODULE_3__ = /* #__PURE__ */ __webpack_require__(/*! react-redux */ "webpack/sharing/consume/default/react-redux/react-redux");
/* ESM import */var react_redux__WEBPACK_IMPORTED_MODULE_3___default = /*#__PURE__*/__webpack_require__.n(react_redux__WEBPACK_IMPORTED_MODULE_3__);
/* ESM import */var _store_slices_dashboardSlice__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ../store/slices/dashboardSlice */ "./src/store/slices/dashboardSlice.js");





var Title = antd__WEBPACK_IMPORTED_MODULE_1__.Typography.Title;
var Dashboard = function() {
    var dispatch = (0,react_redux__WEBPACK_IMPORTED_MODULE_3__.useDispatch)();
    var _useSelector = (0,react_redux__WEBPACK_IMPORTED_MODULE_3__.useSelector)(function(state) {
        return state.dashboard;
    }), stats = _useSelector.stats, loading = _useSelector.loading;
    (0,react__WEBPACK_IMPORTED_MODULE_0__.useEffect)(function() {
        dispatch((0,_store_slices_dashboardSlice__WEBPACK_IMPORTED_MODULE_4__.fetchDashboardStats)());
    }, [
        dispatch
    ]);
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 2
    }, "Dashboard"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Row, {
        gutter: [
            16,
            16
        ],
        className: "dashboard-stats"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        sm: 12,
        lg: 6
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        loading: loading
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Statistic, {
        title: "Total Users",
        value: (stats === null || stats === void 0 ? void 0 : stats.totalUsers) || 0,
        prefix: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.UserOutlined, null),
        valueStyle: {
            color: '#3f8600'
        }
    }))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        sm: 12,
        lg: 6
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        loading: loading
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Statistic, {
        title: "Active Users",
        value: (stats === null || stats === void 0 ? void 0 : stats.activeUsers) || 0,
        prefix: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.UserOutlined, null),
        suffix: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("span", {
            style: {
                fontSize: 14,
                color: '#3f8600'
            }
        }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.ArrowUpOutlined, null), " 8%")
    }))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        sm: 12,
        lg: 6
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        loading: loading
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Statistic, {
        title: "Revenue",
        value: (stats === null || stats === void 0 ? void 0 : stats.revenue) || 0,
        prefix: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.DollarOutlined, null),
        precision: 2,
        valueStyle: {
            color: '#1890ff'
        }
    }))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        sm: 12,
        lg: 6
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        loading: loading
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Statistic, {
        title: "Growth",
        value: (stats === null || stats === void 0 ? void 0 : stats.growth) || 0,
        suffix: "%",
        valueStyle: {
            color: (stats === null || stats === void 0 ? void 0 : stats.growth) > 0 ? '#3f8600' : '#cf1322'
        },
        prefix: (stats === null || stats === void 0 ? void 0 : stats.growth) > 0 ? /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.ArrowUpOutlined, null) : /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.ArrowDownOutlined, null)
    })))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Row, {
        gutter: [
            16,
            16
        ]
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        lg: 16
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: "Recent Activity",
        loading: loading
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        direction: "vertical",
        style: {
            width: '100%'
        }
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, "User John Doe completed a purchase - $125.00"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, "New user registration: jane.smith@example.com"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, "System maintenance scheduled for next week"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, "Revenue target achieved for Q4")))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        lg: 8
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: "Quick Stats",
        loading: loading
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        direction: "vertical",
        style: {
            width: '100%'
        }
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, "Conversion Rate: 3.2%"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, "Avg. Session Duration: 5m 32s"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, "Bounce Rate: 42%"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, "Page Views: 125,432"))))));
};
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (Dashboard);


}),
"./src/pages/RemoteShowcase.jsx": 
/*!**************************************!*\
  !*** ./src/pages/RemoteShowcase.jsx ***!
  \**************************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__)
});
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! react */ "webpack/sharing/consume/default/react/react");
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_1__);
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! @ant-design/icons */ "webpack/sharing/consume/default/@ant-design/icons/@ant-design/icons");
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__);
function _array_like_to_array(arr, len) {
    if (len == null || len > arr.length) len = arr.length;
    for(var i = 0, arr2 = new Array(len); i < len; i++)arr2[i] = arr[i];
    return arr2;
}
function _array_with_holes(arr) {
    if (Array.isArray(arr)) return arr;
}
function _iterable_to_array_limit(arr, i) {
    var _i = arr == null ? null : typeof Symbol !== "undefined" && arr[Symbol.iterator] || arr["@@iterator"];
    if (_i == null) return;
    var _arr = [];
    var _n = true;
    var _d = false;
    var _s, _e;
    try {
        for(_i = _i.call(arr); !(_n = (_s = _i.next()).done); _n = true){
            _arr.push(_s.value);
            if (i && _arr.length === i) break;
        }
    } catch (err) {
        _d = true;
        _e = err;
    } finally{
        try {
            if (!_n && _i["return"] != null) _i["return"]();
        } finally{
            if (_d) throw _e;
        }
    }
    return _arr;
}
function _non_iterable_rest() {
    throw new TypeError("Invalid attempt to destructure non-iterable instance.\\nIn order to be iterable, non-array objects must have a [Symbol.iterator]() method.");
}
function _sliced_to_array(arr, i) {
    return _array_with_holes(arr) || _iterable_to_array_limit(arr, i) || _unsupported_iterable_to_array(arr, i) || _non_iterable_rest();
}
function _unsupported_iterable_to_array(o, minLen) {
    if (!o) return;
    if (typeof o === "string") return _array_like_to_array(o, minLen);
    var n = Object.prototype.toString.call(o).slice(8, -1);
    if (n === "Object" && o.constructor) n = o.constructor.name;
    if (n === "Map" || n === "Set") return Array.from(n);
    if (n === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(n)) return _array_like_to_array(o, minLen);
}



var Title = antd__WEBPACK_IMPORTED_MODULE_1__.Typography.Title;
// Lazy load remote components
var UserCard = /*#__PURE__*/ (0,react__WEBPACK_IMPORTED_MODULE_0__.lazy)(function() {
    return __webpack_require__.e(/*! import() */ "webpack_container_remote_remote_UserCard").then(__webpack_require__.t.bind(__webpack_require__, /*! remote/UserCard */ "webpack/container/remote/remote/UserCard", 23));
});
var DataTable = /*#__PURE__*/ (0,react__WEBPACK_IMPORTED_MODULE_0__.lazy)(function() {
    return __webpack_require__.e(/*! import() */ "webpack_container_remote_remote_DataTable").then(__webpack_require__.t.bind(__webpack_require__, /*! remote/DataTable */ "webpack/container/remote/remote/DataTable", 23));
});
var ChartWidget = /*#__PURE__*/ (0,react__WEBPACK_IMPORTED_MODULE_0__.lazy)(function() {
    return __webpack_require__.e(/*! import() */ "webpack_container_remote_remote_ChartWidget").then(__webpack_require__.t.bind(__webpack_require__, /*! remote/ChartWidget */ "webpack/container/remote/remote/ChartWidget", 23));
});
var FormBuilder = /*#__PURE__*/ (0,react__WEBPACK_IMPORTED_MODULE_0__.lazy)(function() {
    return __webpack_require__.e(/*! import() */ "webpack_container_remote_remote_FormBuilder").then(__webpack_require__.t.bind(__webpack_require__, /*! remote/FormBuilder */ "webpack/container/remote/remote/FormBuilder", 23));
});
var RemoteComponentWrapper = function(param) {
    var children = param.children, title = param.title;
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: title,
        className: "remote-component-wrapper"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react__WEBPACK_IMPORTED_MODULE_0__.Suspense, {
        fallback: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
            style: {
                textAlign: 'center',
                padding: 40
            }
        }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Spin, {
            size: "large"
        }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("p", null, "Loading remote component..."))
    }, children));
};
var RemoteShowcase = function() {
    var _useState = _sliced_to_array((0,react__WEBPACK_IMPORTED_MODULE_0__.useState)('1'), 2), activeTab = _useState[0], setActiveTab = _useState[1];
    var tabItems = [
        {
            key: '1',
            label: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("span", null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.AppstoreOutlined, null), "User Card"),
            children: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(RemoteComponentWrapper, {
                title: "User Profile Card Component"
            }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(UserCard, {
                user: {
                    name: 'John Doe',
                    email: 'john.doe@example.com',
                    avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=John',
                    role: 'Senior Developer',
                    department: 'Engineering',
                    joinDate: '2022-01-15'
                }
            }))
        },
        {
            key: '2',
            label: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("span", null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.TableOutlined, null), "Data Table"),
            children: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(RemoteComponentWrapper, {
                title: "Advanced Data Table Component"
            }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(DataTable, null))
        },
        {
            key: '3',
            label: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("span", null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.BarChartOutlined, null), "Charts"),
            children: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(RemoteComponentWrapper, {
                title: "Chart Widgets"
            }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(ChartWidget, {
                type: "line"
            }))
        },
        {
            key: '4',
            label: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("span", null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.FormOutlined, null), "Form Builder"),
            children: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(RemoteComponentWrapper, {
                title: "Dynamic Form Builder"
            }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(FormBuilder, {
                fields: [
                    {
                        type: 'text',
                        name: 'firstName',
                        label: 'First Name',
                        required: true
                    },
                    {
                        type: 'text',
                        name: 'lastName',
                        label: 'Last Name',
                        required: true
                    },
                    {
                        type: 'email',
                        name: 'email',
                        label: 'Email',
                        required: true
                    },
                    {
                        type: 'select',
                        name: 'department',
                        label: 'Department',
                        options: [
                            'Engineering',
                            'Sales',
                            'Marketing',
                            'HR'
                        ]
                    }
                ],
                onSubmit: function(values) {
                    return console.log('Form submitted:', values);
                }
            }))
        }
    ];
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 2
    }, "Remote Components Showcase"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Alert, {
        message: "Module Federation in Action",
        description: "These components are loaded dynamically from a remote application running on port 3002. They share React, Ant Design, and other dependencies with the host application.",
        type: "info",
        showIcon: true,
        style: {
            marginBottom: 24
        }
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Tabs, {
        activeKey: activeTab,
        onChange: setActiveTab,
        items: tabItems
    }));
};
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (RemoteShowcase);


}),
"./src/pages/Settings.jsx": 
/*!********************************!*\
  !*** ./src/pages/Settings.jsx ***!
  \********************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__)
});
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! react */ "webpack/sharing/consume/default/react/react");
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_1__);
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! @ant-design/icons */ "webpack/sharing/consume/default/@ant-design/icons/@ant-design/icons");
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__);
function _array_like_to_array(arr, len) {
    if (len == null || len > arr.length) len = arr.length;
    for(var i = 0, arr2 = new Array(len); i < len; i++)arr2[i] = arr[i];
    return arr2;
}
function _array_with_holes(arr) {
    if (Array.isArray(arr)) return arr;
}
function _iterable_to_array_limit(arr, i) {
    var _i = arr == null ? null : typeof Symbol !== "undefined" && arr[Symbol.iterator] || arr["@@iterator"];
    if (_i == null) return;
    var _arr = [];
    var _n = true;
    var _d = false;
    var _s, _e;
    try {
        for(_i = _i.call(arr); !(_n = (_s = _i.next()).done); _n = true){
            _arr.push(_s.value);
            if (i && _arr.length === i) break;
        }
    } catch (err) {
        _d = true;
        _e = err;
    } finally{
        try {
            if (!_n && _i["return"] != null) _i["return"]();
        } finally{
            if (_d) throw _e;
        }
    }
    return _arr;
}
function _non_iterable_rest() {
    throw new TypeError("Invalid attempt to destructure non-iterable instance.\\nIn order to be iterable, non-array objects must have a [Symbol.iterator]() method.");
}
function _sliced_to_array(arr, i) {
    return _array_with_holes(arr) || _iterable_to_array_limit(arr, i) || _unsupported_iterable_to_array(arr, i) || _non_iterable_rest();
}
function _unsupported_iterable_to_array(o, minLen) {
    if (!o) return;
    if (typeof o === "string") return _array_like_to_array(o, minLen);
    var n = Object.prototype.toString.call(o).slice(8, -1);
    if (n === "Object" && o.constructor) n = o.constructor.name;
    if (n === "Map" || n === "Set") return Array.from(n);
    if (n === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(n)) return _array_like_to_array(o, minLen);
}



var Title = antd__WEBPACK_IMPORTED_MODULE_1__.Typography.Title, Text = antd__WEBPACK_IMPORTED_MODULE_1__.Typography.Text;
var Settings = function() {
    var _Form_useForm = _sliced_to_array(antd__WEBPACK_IMPORTED_MODULE_1__.Form.useForm(), 1), form = _Form_useForm[0];
    var handleSubmit = function(values) {
        console.log('Settings saved:', values);
    // Here you would typically save to backend/state
    };
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 2
    }, "Settings"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Row, {
        gutter: [
            16,
            16
        ]
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        lg: 16
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: "General Settings"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form, {
        form: form,
        layout: "vertical",
        onFinish: handleSubmit,
        initialValues: {
            appName: 'Module Federation React Demo',
            language: 'en',
            timezone: 'UTC',
            notifications: true,
            darkMode: false
        }
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, {
        label: "Application Name",
        name: "appName",
        rules: [
            {
                required: true,
                message: 'Please enter application name'
            }
        ]
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Input, null)), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, {
        label: "Language",
        name: "language"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select.Option, {
        value: "en"
    }, "English"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select.Option, {
        value: "es"
    }, "Spanish"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select.Option, {
        value: "fr"
    }, "French"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select.Option, {
        value: "de"
    }, "German"))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, {
        label: "Timezone",
        name: "timezone"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select.Option, {
        value: "UTC"
    }, "UTC"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select.Option, {
        value: "EST"
    }, "Eastern Time"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select.Option, {
        value: "PST"
    }, "Pacific Time"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select.Option, {
        value: "CET"
    }, "Central European Time"))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Divider, null), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, {
        label: "Email Notifications",
        name: "notifications",
        valuePropName: "checked"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Switch, null)), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, {
        label: "Dark Mode",
        name: "darkMode",
        valuePropName: "checked"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Switch, null)), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Button, {
        type: "primary",
        htmlType: "submit",
        icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.SaveOutlined, null)
    }, "Save Settings"))))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        lg: 8
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: "Application Info"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        direction: "vertical",
        style: {
            width: '100%'
        }
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        strong: true
    }, "Version:"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, null, " 1.0.0")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        strong: true
    }, "Environment:"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, null, " Development")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        strong: true
    }, "API Endpoint:"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, null, " http://localhost:3000/api")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        strong: true
    }, "Build Date:"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, null, " ", new Date().toLocaleDateString())))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: "Module Federation Info",
        style: {
            marginTop: 16
        }
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        direction: "vertical",
        style: {
            width: '100%'
        }
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        strong: true
    }, "Host URL:"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, null, " http://localhost:3001")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        strong: true
    }, "Remote URL:"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, null, " http://localhost:3002")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        strong: true
    }, "Shared Libs:"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, null, " React, Ant Design, Redux")))))));
};
// Import Row and Col

/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (Settings);


}),
"./src/pages/Users.jsx": 
/*!*****************************!*\
  !*** ./src/pages/Users.jsx ***!
  \*****************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__)
});
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! react */ "webpack/sharing/consume/default/react/react");
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_1__);
/* ESM import */var react_redux__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! react-redux */ "webpack/sharing/consume/default/react-redux/react-redux");
/* ESM import */var react_redux__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(react_redux__WEBPACK_IMPORTED_MODULE_2__);
/* ESM import */var _store_slices_usersSlice__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ../store/slices/usersSlice */ "./src/store/slices/usersSlice.js");
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_4__ = /* #__PURE__ */ __webpack_require__(/*! @ant-design/icons */ "webpack/sharing/consume/default/@ant-design/icons/@ant-design/icons");
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_4___default = /*#__PURE__*/__webpack_require__.n(_ant_design_icons__WEBPACK_IMPORTED_MODULE_4__);





var Title = antd__WEBPACK_IMPORTED_MODULE_1__.Typography.Title;
var Users = function() {
    var dispatch = (0,react_redux__WEBPACK_IMPORTED_MODULE_2__.useDispatch)();
    var _useSelector = (0,react_redux__WEBPACK_IMPORTED_MODULE_2__.useSelector)(function(state) {
        return state.users;
    }), list = _useSelector.list, loading = _useSelector.loading, sortField = _useSelector.sortField;
    (0,react__WEBPACK_IMPORTED_MODULE_0__.useEffect)(function() {
        dispatch((0,_store_slices_usersSlice__WEBPACK_IMPORTED_MODULE_3__.fetchUsers)());
    }, [
        dispatch
    ]);
    var handleSortChange = function(value) {
        dispatch((0,_store_slices_usersSlice__WEBPACK_IMPORTED_MODULE_3__.sortUsers)(value));
    };
    var columns = [
        {
            title: 'Name',
            dataIndex: 'name',
            key: 'name'
        },
        {
            title: 'Email',
            dataIndex: 'email',
            key: 'email'
        },
        {
            title: 'Role',
            dataIndex: 'role',
            key: 'role',
            render: function(role) {
                var color = role === 'Admin' ? 'red' : role === 'Manager' ? 'blue' : 'green';
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Tag, {
                    color: color
                }, role);
            }
        },
        {
            title: 'Status',
            dataIndex: 'status',
            key: 'status',
            render: function(status) {
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Tag, {
                    color: status === 'active' ? 'green' : 'default'
                }, status.toUpperCase());
            }
        },
        {
            title: 'Actions',
            key: 'actions',
            render: function(_, record) {
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
                    size: "middle"
                }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Button, {
                    type: "link",
                    icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_4__.EditOutlined, null),
                    onClick: function() {
                        return console.log('Edit', record);
                    }
                }, "Edit"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Button, {
                    type: "link",
                    danger: true,
                    icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_4__.DeleteOutlined, null),
                    onClick: function() {
                        return console.log('Delete', record);
                    }
                }, "Delete"));
            }
        }
    ];
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Row, {
        justify: "space-between",
        align: "middle",
        style: {
            marginBottom: 24
        }
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 2
    }, "Users")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("span", null, "Sort by:"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select, {
        value: sortField,
        onChange: handleSortChange,
        style: {
            width: 150
        },
        options: [
            {
                value: 'name',
                label: 'Name'
            },
            {
                value: 'email',
                label: 'Email'
            },
            {
                value: 'role',
                label: 'Role'
            }
        ]
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Button, {
        type: "primary"
    }, "Add User")))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Table, {
        columns: columns,
        dataSource: list,
        loading: loading,
        rowKey: "id",
        pagination: {
            pageSize: 10,
            showTotal: function(total) {
                return "Total ".concat(total, " users");
            }
        }
    }));
};
// Import Row and Col

/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (Users);


}),
"../../../node_modules/.pnpm/react-dom@18.3.1_react@18.3.1/node_modules/react-dom/client.js": 
/*!**************************************************************************************************!*\
  !*** ../../../node_modules/.pnpm/react-dom@18.3.1_react@18.3.1/node_modules/react-dom/client.js ***!
  \**************************************************************************************************/
(function (__unused_webpack_module, exports, __webpack_require__) {

var m = __webpack_require__(/*! react-dom */ "webpack/sharing/consume/default/react-dom/react-dom");
if (false) {} else {
    var i = m.__SECRET_INTERNALS_DO_NOT_USE_OR_YOU_WILL_BE_FIRED;
    exports.createRoot = function(c, o) {
        i.usingClientEntryPoint = true;
        try {
            return m.createRoot(c, o);
        } finally{
            i.usingClientEntryPoint = false;
        }
    };
    exports.hydrateRoot = function(c, h, o) {
        i.usingClientEntryPoint = true;
        try {
            return m.hydrateRoot(c, h, o);
        } finally{
            i.usingClientEntryPoint = false;
        }
    };
}


}),
"./src/store/index.js": 
/*!****************************!*\
  !*** ./src/store/index.js ***!
  \****************************/
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  store: () => (store)
});
/* ESM import */var _reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! @reduxjs/toolkit */ "webpack/sharing/consume/default/@reduxjs/toolkit/@reduxjs/toolkit");
/* ESM import */var _slices_dashboardSlice_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./slices/dashboardSlice.js */ "./src/store/slices/dashboardSlice.js");
/* ESM import */var _slices_usersSlice_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./slices/usersSlice.js */ "./src/store/slices/usersSlice.js");
/* ESM import */var _slices_analyticsSlice_js__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./slices/analyticsSlice.js */ "./src/store/slices/analyticsSlice.js");




var store = (0,_reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__.configureStore)({
    reducer: {
        dashboard: _slices_dashboardSlice_js__WEBPACK_IMPORTED_MODULE_1__["default"],
        users: _slices_usersSlice_js__WEBPACK_IMPORTED_MODULE_2__["default"],
        analytics: _slices_analyticsSlice_js__WEBPACK_IMPORTED_MODULE_3__["default"]
    }
});


}),
"./src/store/slices/analyticsSlice.js": 
/*!********************************************!*\
  !*** ./src/store/slices/analyticsSlice.js ***!
  \********************************************/
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__),
  fetchAnalyticsData: () => (fetchAnalyticsData),
  setTimeRange: () => (setTimeRange)
});
/* ESM import */var _reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! @reduxjs/toolkit */ "webpack/sharing/consume/default/@reduxjs/toolkit/@reduxjs/toolkit");
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! lodash-es */ "webpack/sharing/consume/default/lodash-es/lodash-es");
function asyncGeneratorStep(gen, resolve, reject, _next, _throw, key, arg) {
    try {
        var info = gen[key](arg);
        var value = info.value;
    } catch (error) {
        reject(error);
        return;
    }
    if (info.done) {
        resolve(value);
    } else {
        Promise.resolve(value).then(_next, _throw);
    }
}
function _async_to_generator(fn) {
    return function() {
        var self = this, args = arguments;
        return new Promise(function(resolve, reject) {
            var gen = fn.apply(self, args);
            function _next(value) {
                asyncGeneratorStep(gen, resolve, reject, _next, _throw, "next", value);
            }
            function _throw(err) {
                asyncGeneratorStep(gen, resolve, reject, _next, _throw, "throw", err);
            }
            _next(undefined);
        });
    };
}
function _ts_generator(thisArg, body) {
    var f, y, t, _ = {
        label: 0,
        sent: function() {
            if (t[0] & 1) throw t[1];
            return t[1];
        },
        trys: [],
        ops: []
    }, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() {
        return this;
    }), g;
    function verb(n) {
        return function(v) {
            return step([
                n,
                v
            ]);
        };
    }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while(g && (g = 0, op[0] && (_ = 0)), _)try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [
                op[0] & 2,
                t.value
            ];
            switch(op[0]){
                case 0:
                case 1:
                    t = op;
                    break;
                case 4:
                    _.label++;
                    return {
                        value: op[1],
                        done: false
                    };
                case 5:
                    _.label++;
                    y = op[1];
                    op = [
                        0
                    ];
                    continue;
                case 7:
                    op = _.ops.pop();
                    _.trys.pop();
                    continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) {
                        _ = 0;
                        continue;
                    }
                    if (op[0] === 3 && (!t || op[1] > t[0] && op[1] < t[3])) {
                        _.label = op[1];
                        break;
                    }
                    if (op[0] === 6 && _.label < t[1]) {
                        _.label = t[1];
                        t = op;
                        break;
                    }
                    if (t && _.label < t[2]) {
                        _.label = t[2];
                        _.ops.push(op);
                        break;
                    }
                    if (t[2]) _.ops.pop();
                    _.trys.pop();
                    continue;
            }
            op = body.call(thisArg, _);
        } catch (e) {
            op = [
                6,
                e
            ];
            y = 0;
        } finally{
            f = t = 0;
        }
        if (op[0] & 5) throw op[1];
        return {
            value: op[0] ? op[1] : void 0,
            done: true
        };
    }
}


// Simulate API call
var fetchAnalyticsData = (0,_reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__.createAsyncThunk)('analytics/fetchData', function() {
    return _async_to_generator(function() {
        var months, generateData;
        return _ts_generator(this, function(_state) {
            switch(_state.label){
                case 0:
                    return [
                        4,
                        (0,lodash_es__WEBPACK_IMPORTED_MODULE_1__.delay)(600)
                    ];
                case 1:
                    _state.sent();
                    months = [
                        'Jan',
                        'Feb',
                        'Mar',
                        'Apr',
                        'May',
                        'Jun'
                    ];
                    generateData = function() {
                        return months.map(function() {
                            return (0,lodash_es__WEBPACK_IMPORTED_MODULE_1__.random)(100, 500);
                        });
                    };
                    return [
                        2,
                        {
                            revenue: {
                                labels: months,
                                datasets: [
                                    {
                                        label: 'Revenue',
                                        data: generateData(),
                                        borderColor: 'rgb(75, 192, 192)',
                                        backgroundColor: 'rgba(75, 192, 192, 0.2)'
                                    }
                                ]
                            },
                            userGrowth: {
                                labels: months,
                                datasets: [
                                    {
                                        label: 'New Users',
                                        data: generateData(),
                                        borderColor: 'rgb(54, 162, 235)',
                                        backgroundColor: 'rgba(54, 162, 235, 0.2)'
                                    }
                                ]
                            },
                            categories: {
                                labels: [
                                    'Desktop',
                                    'Mobile',
                                    'Tablet'
                                ],
                                datasets: [
                                    {
                                        data: [
                                            (0,lodash_es__WEBPACK_IMPORTED_MODULE_1__.random)(30, 50),
                                            (0,lodash_es__WEBPACK_IMPORTED_MODULE_1__.random)(30, 50),
                                            (0,lodash_es__WEBPACK_IMPORTED_MODULE_1__.random)(10, 20)
                                        ],
                                        backgroundColor: [
                                            'rgba(255, 99, 132, 0.6)',
                                            'rgba(54, 162, 235, 0.6)',
                                            'rgba(255, 206, 86, 0.6)'
                                        ]
                                    }
                                ]
                            }
                        }
                    ];
            }
        });
    })();
});
var analyticsSlice = (0,_reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__.createSlice)({
    name: 'analytics',
    initialState: {
        data: null,
        loading: false,
        error: null,
        timeRange: '6months'
    },
    reducers: {
        setTimeRange: function(state, action) {
            state.timeRange = action.payload;
        }
    },
    extraReducers: function(builder) {
        builder.addCase(fetchAnalyticsData.pending, function(state) {
            state.loading = true;
            state.error = null;
        }).addCase(fetchAnalyticsData.fulfilled, function(state, action) {
            state.loading = false;
            state.data = action.payload;
        }).addCase(fetchAnalyticsData.rejected, function(state, action) {
            state.loading = false;
            state.error = action.error.message;
        });
    }
});
var setTimeRange = analyticsSlice.actions.setTimeRange;
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (analyticsSlice.reducer);


}),
"./src/store/slices/dashboardSlice.js": 
/*!********************************************!*\
  !*** ./src/store/slices/dashboardSlice.js ***!
  \********************************************/
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__),
  fetchDashboardStats: () => (fetchDashboardStats),
  resetDashboard: () => (resetDashboard)
});
/* ESM import */var _reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! @reduxjs/toolkit */ "webpack/sharing/consume/default/@reduxjs/toolkit/@reduxjs/toolkit");
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! lodash-es */ "webpack/sharing/consume/default/lodash-es/lodash-es");
function asyncGeneratorStep(gen, resolve, reject, _next, _throw, key, arg) {
    try {
        var info = gen[key](arg);
        var value = info.value;
    } catch (error) {
        reject(error);
        return;
    }
    if (info.done) {
        resolve(value);
    } else {
        Promise.resolve(value).then(_next, _throw);
    }
}
function _async_to_generator(fn) {
    return function() {
        var self = this, args = arguments;
        return new Promise(function(resolve, reject) {
            var gen = fn.apply(self, args);
            function _next(value) {
                asyncGeneratorStep(gen, resolve, reject, _next, _throw, "next", value);
            }
            function _throw(err) {
                asyncGeneratorStep(gen, resolve, reject, _next, _throw, "throw", err);
            }
            _next(undefined);
        });
    };
}
function _ts_generator(thisArg, body) {
    var f, y, t, _ = {
        label: 0,
        sent: function() {
            if (t[0] & 1) throw t[1];
            return t[1];
        },
        trys: [],
        ops: []
    }, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() {
        return this;
    }), g;
    function verb(n) {
        return function(v) {
            return step([
                n,
                v
            ]);
        };
    }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while(g && (g = 0, op[0] && (_ = 0)), _)try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [
                op[0] & 2,
                t.value
            ];
            switch(op[0]){
                case 0:
                case 1:
                    t = op;
                    break;
                case 4:
                    _.label++;
                    return {
                        value: op[1],
                        done: false
                    };
                case 5:
                    _.label++;
                    y = op[1];
                    op = [
                        0
                    ];
                    continue;
                case 7:
                    op = _.ops.pop();
                    _.trys.pop();
                    continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) {
                        _ = 0;
                        continue;
                    }
                    if (op[0] === 3 && (!t || op[1] > t[0] && op[1] < t[3])) {
                        _.label = op[1];
                        break;
                    }
                    if (op[0] === 6 && _.label < t[1]) {
                        _.label = t[1];
                        t = op;
                        break;
                    }
                    if (t && _.label < t[2]) {
                        _.label = t[2];
                        _.ops.push(op);
                        break;
                    }
                    if (t[2]) _.ops.pop();
                    _.trys.pop();
                    continue;
            }
            op = body.call(thisArg, _);
        } catch (e) {
            op = [
                6,
                e
            ];
            y = 0;
        } finally{
            f = t = 0;
        }
        if (op[0] & 5) throw op[1];
        return {
            value: op[0] ? op[1] : void 0,
            done: true
        };
    }
}


// Simulate API call
var fetchDashboardStats = (0,_reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__.createAsyncThunk)('dashboard/fetchStats', function() {
    return _async_to_generator(function() {
        return _ts_generator(this, function(_state) {
            switch(_state.label){
                case 0:
                    return [
                        4,
                        (0,lodash_es__WEBPACK_IMPORTED_MODULE_1__.delay)(1000)
                    ];
                case 1:
                    _state.sent();
                    return [
                        2,
                        {
                            totalUsers: 12543,
                            activeUsers: 8921,
                            revenue: 458320,
                            growth: 12.5
                        }
                    ];
            }
        });
    })();
});
var dashboardSlice = (0,_reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__.createSlice)({
    name: 'dashboard',
    initialState: {
        stats: null,
        loading: false,
        error: null
    },
    reducers: {
        resetDashboard: function(state) {
            state.stats = null;
            state.loading = false;
            state.error = null;
        }
    },
    extraReducers: function(builder) {
        builder.addCase(fetchDashboardStats.pending, function(state) {
            state.loading = true;
            state.error = null;
        }).addCase(fetchDashboardStats.fulfilled, function(state, action) {
            state.loading = false;
            state.stats = action.payload;
        }).addCase(fetchDashboardStats.rejected, function(state, action) {
            state.loading = false;
            state.error = action.error.message;
        });
    }
});
var resetDashboard = dashboardSlice.actions.resetDashboard;
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (dashboardSlice.reducer);


}),
"./src/store/slices/usersSlice.js": 
/*!****************************************!*\
  !*** ./src/store/slices/usersSlice.js ***!
  \****************************************/
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__),
  fetchUsers: () => (fetchUsers),
  sortUsers: () => (sortUsers),
  updateUserStatus: () => (updateUserStatus)
});
/* ESM import */var _reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! @reduxjs/toolkit */ "webpack/sharing/consume/default/@reduxjs/toolkit/@reduxjs/toolkit");
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! lodash-es */ "webpack/sharing/consume/default/lodash-es/lodash-es");
function asyncGeneratorStep(gen, resolve, reject, _next, _throw, key, arg) {
    try {
        var info = gen[key](arg);
        var value = info.value;
    } catch (error) {
        reject(error);
        return;
    }
    if (info.done) {
        resolve(value);
    } else {
        Promise.resolve(value).then(_next, _throw);
    }
}
function _async_to_generator(fn) {
    return function() {
        var self = this, args = arguments;
        return new Promise(function(resolve, reject) {
            var gen = fn.apply(self, args);
            function _next(value) {
                asyncGeneratorStep(gen, resolve, reject, _next, _throw, "next", value);
            }
            function _throw(err) {
                asyncGeneratorStep(gen, resolve, reject, _next, _throw, "throw", err);
            }
            _next(undefined);
        });
    };
}
function _ts_generator(thisArg, body) {
    var f, y, t, _ = {
        label: 0,
        sent: function() {
            if (t[0] & 1) throw t[1];
            return t[1];
        },
        trys: [],
        ops: []
    }, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() {
        return this;
    }), g;
    function verb(n) {
        return function(v) {
            return step([
                n,
                v
            ]);
        };
    }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while(g && (g = 0, op[0] && (_ = 0)), _)try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [
                op[0] & 2,
                t.value
            ];
            switch(op[0]){
                case 0:
                case 1:
                    t = op;
                    break;
                case 4:
                    _.label++;
                    return {
                        value: op[1],
                        done: false
                    };
                case 5:
                    _.label++;
                    y = op[1];
                    op = [
                        0
                    ];
                    continue;
                case 7:
                    op = _.ops.pop();
                    _.trys.pop();
                    continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) {
                        _ = 0;
                        continue;
                    }
                    if (op[0] === 3 && (!t || op[1] > t[0] && op[1] < t[3])) {
                        _.label = op[1];
                        break;
                    }
                    if (op[0] === 6 && _.label < t[1]) {
                        _.label = t[1];
                        t = op;
                        break;
                    }
                    if (t && _.label < t[2]) {
                        _.label = t[2];
                        _.ops.push(op);
                        break;
                    }
                    if (t[2]) _.ops.pop();
                    _.trys.pop();
                    continue;
            }
            op = body.call(thisArg, _);
        } catch (e) {
            op = [
                6,
                e
            ];
            y = 0;
        } finally{
            f = t = 0;
        }
        if (op[0] & 5) throw op[1];
        return {
            value: op[0] ? op[1] : void 0,
            done: true
        };
    }
}


// Simulate API call
var fetchUsers = (0,_reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__.createAsyncThunk)('users/fetchUsers', function() {
    return _async_to_generator(function() {
        return _ts_generator(this, function(_state) {
            switch(_state.label){
                case 0:
                    return [
                        4,
                        (0,lodash_es__WEBPACK_IMPORTED_MODULE_1__.delay)(800)
                    ];
                case 1:
                    _state.sent();
                    return [
                        2,
                        [
                            {
                                id: 1,
                                name: 'John Doe',
                                email: 'john@example.com',
                                role: 'Admin',
                                status: 'active'
                            },
                            {
                                id: 2,
                                name: 'Jane Smith',
                                email: 'jane@example.com',
                                role: 'User',
                                status: 'active'
                            },
                            {
                                id: 3,
                                name: 'Bob Johnson',
                                email: 'bob@example.com',
                                role: 'User',
                                status: 'inactive'
                            },
                            {
                                id: 4,
                                name: 'Alice Brown',
                                email: 'alice@example.com',
                                role: 'Manager',
                                status: 'active'
                            },
                            {
                                id: 5,
                                name: 'Charlie Wilson',
                                email: 'charlie@example.com',
                                role: 'User',
                                status: 'active'
                            }
                        ]
                    ];
            }
        });
    })();
});
var usersSlice = (0,_reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__.createSlice)({
    name: 'users',
    initialState: {
        list: [],
        loading: false,
        error: null,
        sortField: 'name'
    },
    reducers: {
        sortUsers: function(state, action) {
            state.sortField = action.payload;
            state.list = (0,lodash_es__WEBPACK_IMPORTED_MODULE_1__.sortBy)(state.list, [
                action.payload
            ]);
        },
        updateUserStatus: function(state, action) {
            var user = state.list.find(function(u) {
                return u.id === action.payload.id;
            });
            if (user) {
                user.status = action.payload.status;
            }
        }
    },
    extraReducers: function(builder) {
        builder.addCase(fetchUsers.pending, function(state) {
            state.loading = true;
            state.error = null;
        }).addCase(fetchUsers.fulfilled, function(state, action) {
            state.loading = false;
            state.list = (0,lodash_es__WEBPACK_IMPORTED_MODULE_1__.sortBy)(action.payload, [
                state.sortField
            ]);
        }).addCase(fetchUsers.rejected, function(state, action) {
            state.loading = false;
            state.error = action.error.message;
        });
    }
});
var _usersSlice_actions = usersSlice.actions;
var sortUsers = _usersSlice_actions.sortUsers, updateUserStatus = _usersSlice_actions.updateUserStatus;
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (usersSlice.reducer);


}),

}]);