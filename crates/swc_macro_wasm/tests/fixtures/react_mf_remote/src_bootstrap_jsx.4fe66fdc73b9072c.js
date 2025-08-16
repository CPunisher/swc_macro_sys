"use strict";
(self["webpackChunk_mf_react_remote"] = self["webpackChunk_mf_react_remote"] || []).push([["src_bootstrap_jsx"], {
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
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_1__);
/* ESM import */var _components_UserCard__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./components/UserCard */ "./src/components/UserCard.jsx");
/* ESM import */var _components_DataTable__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./components/DataTable */ "./src/components/DataTable.jsx");
/* ESM import */var _components_ChartWidget__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ./components/ChartWidget */ "./src/components/ChartWidget.jsx");
/* ESM import */var _components_FormBuilder__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ./components/FormBuilder */ "./src/components/FormBuilder.jsx");






var Title = antd__WEBPACK_IMPORTED_MODULE_1__.Typography.Title;
var App = function() {
    var handleFormSubmit = function(values) {
        console.log('Form submitted:', values);
    };
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 1
    }, "Remote Components Library"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Alert, {
        message: "Module Federation Remote App",
        description: "This app exposes React components that can be consumed by other applications. All components below are available for remote consumption.",
        type: "info",
        showIcon: true,
        style: {
            marginBottom: 32
        }
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        direction: "vertical",
        size: "large",
        style: {
            width: '100%'
        }
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        className: "component-demo"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 3,
        className: "component-title"
    }, "UserCard Component"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_components_UserCard__WEBPACK_IMPORTED_MODULE_2__["default"], {
        user: {
            name: 'Jane Doe',
            email: 'jane.doe@example.com',
            avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Jane',
            role: 'Product Manager',
            department: 'Product',
            joinDate: '2021-06-15'
        }
    })), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        className: "component-demo"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 3,
        className: "component-title"
    }, "DataTable Component"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_components_DataTable__WEBPACK_IMPORTED_MODULE_3__["default"], null)), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        className: "component-demo"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 3,
        className: "component-title"
    }, "ChartWidget Component"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_components_ChartWidget__WEBPACK_IMPORTED_MODULE_4__["default"], {
        type: "bar"
    })), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        className: "component-demo"
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 3,
        className: "component-title"
    }, "FormBuilder Component"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_components_FormBuilder__WEBPACK_IMPORTED_MODULE_5__["default"], {
        fields: [
            {
                type: 'text',
                name: 'username',
                label: 'Username',
                required: true
            },
            {
                type: 'email',
                name: 'email',
                label: 'Email',
                required: true
            },
            {
                type: 'password',
                name: 'password',
                label: 'Password',
                required: true
            },
            {
                type: 'select',
                name: 'role',
                label: 'Role',
                options: [
                    'Admin',
                    'User',
                    'Guest'
                ],
                required: true
            }
        ],
        onSubmit: handleFormSubmit
    }))));
};
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
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_2__);
/* ESM import */var _App__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./App */ "./src/App.jsx");




var root = react_dom_client__WEBPACK_IMPORTED_MODULE_1__.createRoot(document.getElementById('root'));
root.render(/*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement((react__WEBPACK_IMPORTED_MODULE_0___default().StrictMode), null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_2__.ConfigProvider, {
    theme: {
        token: {
            colorPrimary: '#1890ff',
            borderRadius: 6
        }
    }
}, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_App__WEBPACK_IMPORTED_MODULE_3__["default"], null))));


}),
"./src/components/ChartWidget.jsx": 
/*!****************************************!*\
  !*** ./src/components/ChartWidget.jsx ***!
  \****************************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__)
});
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! react */ "webpack/sharing/consume/default/react/react");
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var react_chartjs_2__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! react-chartjs-2 */ "webpack/sharing/consume/default/react-chartjs-2/react-chartjs-2");
/* ESM import */var react_chartjs_2__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(react_chartjs_2__WEBPACK_IMPORTED_MODULE_1__);
/* ESM import */var chart_js__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! chart.js */ "webpack/sharing/consume/default/chart.js/chart.js");
/* ESM import */var chart_js__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(chart_js__WEBPACK_IMPORTED_MODULE_2__);
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_3__ = /* #__PURE__ */ __webpack_require__(/*! lodash-es */ "webpack/sharing/consume/default/lodash-es/lodash-es");
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_3___default = /*#__PURE__*/__webpack_require__.n(lodash_es__WEBPACK_IMPORTED_MODULE_3__);




chart_js__WEBPACK_IMPORTED_MODULE_2__.Chart.register(chart_js__WEBPACK_IMPORTED_MODULE_2__.CategoryScale, chart_js__WEBPACK_IMPORTED_MODULE_2__.LinearScale, chart_js__WEBPACK_IMPORTED_MODULE_2__.PointElement, chart_js__WEBPACK_IMPORTED_MODULE_2__.LineElement, chart_js__WEBPACK_IMPORTED_MODULE_2__.BarElement, chart_js__WEBPACK_IMPORTED_MODULE_2__.ArcElement, chart_js__WEBPACK_IMPORTED_MODULE_2__.Title, chart_js__WEBPACK_IMPORTED_MODULE_2__.Tooltip, chart_js__WEBPACK_IMPORTED_MODULE_2__.Legend);
var ChartWidget = function(param) {
    var _param_type = param.type, type = _param_type === void 0 ? 'line' : _param_type, propData = param.data, propOptions = param.options, _param_height = param.height, height = _param_height === void 0 ? 300 : _param_height;
    var defaultData = (0,react__WEBPACK_IMPORTED_MODULE_0__.useMemo)(function() {
        var labels = [
            'January',
            'February',
            'March',
            'April',
            'May',
            'June',
            'July'
        ];
        var dataset1 = labels.map(function() {
            return (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.random)(10, 100);
        });
        var dataset2 = labels.map(function() {
            return (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.random)(10, 100);
        });
        switch(type){
            case 'pie':
            case 'doughnut':
                return {
                    labels: [
                        'Red',
                        'Blue',
                        'Yellow',
                        'Green',
                        'Purple'
                    ],
                    datasets: [
                        {
                            data: [
                                (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.random)(10, 50),
                                (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.random)(10, 50),
                                (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.random)(10, 50),
                                (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.random)(10, 50),
                                (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.random)(10, 50)
                            ],
                            backgroundColor: [
                                'rgba(255, 99, 132, 0.6)',
                                'rgba(54, 162, 235, 0.6)',
                                'rgba(255, 206, 86, 0.6)',
                                'rgba(75, 192, 192, 0.6)',
                                'rgba(153, 102, 255, 0.6)'
                            ],
                            borderColor: [
                                'rgba(255, 99, 132, 1)',
                                'rgba(54, 162, 235, 1)',
                                'rgba(255, 206, 86, 1)',
                                'rgba(75, 192, 192, 1)',
                                'rgba(153, 102, 255, 1)'
                            ],
                            borderWidth: 1
                        }
                    ]
                };
            default:
                return {
                    labels: labels,
                    datasets: [
                        {
                            label: 'Dataset 1',
                            data: dataset1,
                            borderColor: 'rgb(255, 99, 132)',
                            backgroundColor: 'rgba(255, 99, 132, 0.5)',
                            tension: 0.1
                        },
                        {
                            label: 'Dataset 2',
                            data: dataset2,
                            borderColor: 'rgb(53, 162, 235)',
                            backgroundColor: 'rgba(53, 162, 235, 0.5)',
                            tension: 0.1
                        }
                    ]
                };
        }
    }, [
        type
    ]);
    var defaultOptions = {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
            legend: {
                position: 'top'
            },
            title: {
                display: true,
                text: "Sample ".concat(type.charAt(0).toUpperCase() + type.slice(1), " Chart")
            }
        }
    };
    var chartData = propData || defaultData;
    var chartOptions = propOptions || defaultOptions;
    var renderChart = function() {
        var props = {
            data: chartData,
            options: chartOptions,
            height: height
        };
        switch(type){
            case 'line':
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_chartjs_2__WEBPACK_IMPORTED_MODULE_1__.Line, props);
            case 'bar':
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_chartjs_2__WEBPACK_IMPORTED_MODULE_1__.Bar, props);
            case 'pie':
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_chartjs_2__WEBPACK_IMPORTED_MODULE_1__.Pie, props);
            case 'doughnut':
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_chartjs_2__WEBPACK_IMPORTED_MODULE_1__.Doughnut, props);
            default:
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_chartjs_2__WEBPACK_IMPORTED_MODULE_1__.Line, props);
        }
    };
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        style: {
            height: height,
            position: 'relative'
        }
    }, renderChart());
};
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (ChartWidget);


}),
"./src/components/UserCard.jsx": 
/*!*************************************!*\
  !*** ./src/components/UserCard.jsx ***!
  \*************************************/
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
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_3__ = /* #__PURE__ */ __webpack_require__(/*! lodash-es */ "webpack/sharing/consume/default/lodash-es/lodash-es");
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_3___default = /*#__PURE__*/__webpack_require__.n(lodash_es__WEBPACK_IMPORTED_MODULE_3__);
/* ESM import */var dayjs__WEBPACK_IMPORTED_MODULE_4__ = /* #__PURE__ */ __webpack_require__(/*! dayjs */ "webpack/sharing/consume/default/dayjs/dayjs");
/* ESM import */var dayjs__WEBPACK_IMPORTED_MODULE_4___default = /*#__PURE__*/__webpack_require__.n(dayjs__WEBPACK_IMPORTED_MODULE_4__);





var Text = antd__WEBPACK_IMPORTED_MODULE_1__.Typography.Text, Title = antd__WEBPACK_IMPORTED_MODULE_1__.Typography.Title;
var UserCard = function(param) {
    var user = param.user;
    var _ref = user || {}, _ref_name = _ref.name, name = _ref_name === void 0 ? 'Unknown User' : _ref_name, _ref_email = _ref.email, email = _ref_email === void 0 ? 'no-email@example.com' : _ref_email, avatar = _ref.avatar, _ref_role = _ref.role, role = _ref_role === void 0 ? 'User' : _ref_role, _ref_department = _ref.department, department = _ref_department === void 0 ? 'General' : _ref_department, _ref_joinDate = _ref.joinDate, joinDate = _ref_joinDate === void 0 ? new Date().toISOString() : _ref_joinDate;
    var formatJoinDate = dayjs__WEBPACK_IMPORTED_MODULE_4___default()(joinDate).format('MMMM D, YYYY');
    var yearsOfService = dayjs__WEBPACK_IMPORTED_MODULE_4___default()().diff(dayjs__WEBPACK_IMPORTED_MODULE_4___default()(joinDate), 'year');
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        style: {
            width: '100%',
            maxWidth: 400
        }
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        direction: "vertical",
        size: "middle",
        style: {
            width: '100%'
        }
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Row, {
        align: "middle",
        gutter: 16
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Avatar, {
        size: 80,
        src: avatar,
        icon: !avatar && /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.UserOutlined, null),
        style: {
            backgroundColor: '#1890ff'
        }
    })), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 4,
        style: {
            margin: 0
        }
    }, (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.capitalize)(name)), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Tag, {
        color: "blue"
    }, role))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        direction: "vertical",
        style: {
            width: '100%'
        }
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.MailOutlined, null), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, null, email)), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.TeamOutlined, null), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, null, (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.capitalize)(department), " Department")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.CalendarOutlined, null), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, null, "Joined ", formatJoinDate))), yearsOfService > 0 && /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        style: {
            marginTop: 12,
            padding: '8px 12px',
            background: '#f0f2f5',
            borderRadius: 4
        }
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        type: "secondary"
    }, yearsOfService, " ", yearsOfService === 1 ? 'year' : 'years', " with the company"))));
};
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (UserCard);


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

}]);