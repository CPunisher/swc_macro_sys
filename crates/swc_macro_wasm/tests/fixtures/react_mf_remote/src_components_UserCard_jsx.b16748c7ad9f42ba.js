"use strict";
(self["webpackChunk_mf_react_remote"] = self["webpackChunk_mf_react_remote"] || []).push([["src_components_UserCard_jsx"], {
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

}]);