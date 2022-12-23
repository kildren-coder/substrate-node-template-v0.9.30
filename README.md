# Substrate Node Template POE

本仓库为substrate入门课第五期作业.

## 具体实现

主要修改集中于[poe](https://github.com/kildren-coder/substrate-node-template-v0.9.30/tree/main/pallets/poe)模块中,其余的在[runtime](https://github.com/kildren-coder/substrate-node-template-v0.9.30/tree/main/runtime)模块中初始化,并在整个项目的`Cargo.toml`将[poe](https://github.com/kildren-coder/substrate-node-template-v0.9.30/tree/main/pallets/poe)引入.

## BUG说明

在`polkadot.js`中查询链上`POE`模块存储的值时,无法对作为`KEY`的Bytes数据做出反应.除了`0x`,其余输入均只能返回`<unknown>`.所以我们无法查询到单个凭证的信息.

当输入为`0x`时:

![only_ox.png](https://github.com/kildren-coder/substrate-node-template-v0.9.30/blob/main/img/only_0x.png)

此时前端页面会正确地根据输入值`0x`返回`encoded storage key`和`encoded key details`,查询结果也是`Option`的合法返回值:`None`.

可对于除此之外的输入,前端页面既不会返回`encoded storage key`和`encoded key details`的信息,查询结果也是不合法的`unknown`:

![chain_state_bug.png](https://github.com/kildren-coder/substrate-node-template-v0.9.30/blob/main/img/chain_state_bug.png)

好在我们可以通过关闭`included option`选项直接得到整个模块的存储信息.

![pallet_storage.png](https://github.com/kildren-coder/substrate-node-template-v0.9.30/blob/main/img/pallet_storage.png)

由于我们还未添加任何凭证，s所以此时链上是没有相应信息的.

### 尝试过的修复

对于这个BUG,由于不清楚Substrate前后端的联动,所以我有些漫无目的地尝试了修改:
- 修改`StorageMap`的Hash方式,像是`Blake2_128`和`Identify`等.
- 参照官方代码库中v0.9.30分支，修改`BoundedVec`的声明方式.
- 将框架代码升级到最新版本

很可惜这些方法都没有效果.

## 最终效果

### 创建凭证

通过`poeModule`的`createClaim`方法创建凭证:

![creatw_claim.png](https://github.com/kildren-coder/substrate-node-template-v0.9.30/blob/main/img/create_claim.png)

再在chain state中查询该模块的信息:

![create_claim_result.png](https://github.com/kildren-coder/substrate-node-template-v0.9.30/blob/main/img/create_claim_result_1.png)

可以看到已经生成了相应凭证,其持有地址为之前的申请人Alice的地址.

### 转移凭证

先尝试转移不存在的凭证:

![shift_undefined_claim.png](https://github.com/kildren-coder/substrate-node-template-v0.9.30/blob/main/img/shift_undefined_claim.png)

前端返回错误:

![shift_failed.png](https://github.com/kildren-coder/substrate-node-template-v0.9.30/blob/main/img/shift_failed.png)

将之前的凭证转移给Bob:

![shift_claim.png](https://github.com/kildren-coder/substrate-node-template-v0.9.30/blob/main/img/shift_claim.png)

查询可知凭证已转移给Bob:

![shift_result.png](https://github.com/kildren-coder/substrate-node-template-v0.9.30/blob/main/img/shift_result.png)

### 撤销凭证

先尝试让Alice撤销凭证,不过由于凭证已转移,所以Alice不再持有该凭证,无权撤销:

![revoked_by_others.png](https://github.com/kildren-coder/substrate-node-template-v0.9.30/blob/main/img/revoked_by_others.png)

再让Bob撤销凭证:

![revoked_by_owner.png](https://github.com/kildren-coder/substrate-node-template-v0.9.30/blob/main/img/revoked_by_owner.png)

查询可知凭证已被撤销:

![revoked_result.png](https://github.com/kildren-coder/substrate-node-template-v0.9.30/blob/main/img/revoked_result.png)

### 链上记录

所有成功执行的`extrinsics`都可以在`Network`的`Chain info`中查看,其按时间顺序记录了这些调用:

![final_result.png](https://github.com/kildren-coder/substrate-node-template-v0.9.30/blob/main/img/final_result.png)