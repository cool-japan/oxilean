//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ChiselAnalysisCache, ChiselAnnotationKind, ChiselBackend, ChiselConstantFoldingHelper,
    ChiselDepGraph, ChiselDominatorTree, ChiselExpr, ChiselInterfaceTemplate, ChiselLivenessInfo,
    ChiselModule, ChiselPassConfig, ChiselPassPhase, ChiselPassRegistry, ChiselPassStats,
    ChiselPipelineRegisterChain, ChiselPort, ChiselReadyValidBundle, ChiselSRAMWrapper,
    ChiselStreamingModule, ChiselType, ChiselWorklist, PipelineStage, StreamDirection,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_type_uint() {
        assert_eq!(ChiselType::UInt(8).to_string(), "UInt(8.W)");
    }
    #[test]
    pub(super) fn test_type_sint() {
        assert_eq!(ChiselType::SInt(16).to_string(), "SInt(16.W)");
    }
    #[test]
    pub(super) fn test_type_bool() {
        assert_eq!(ChiselType::Bool.to_string(), "Bool()");
    }
    #[test]
    pub(super) fn test_type_clock() {
        assert_eq!(ChiselType::Clock.to_string(), "Clock()");
    }
    #[test]
    pub(super) fn test_type_reset() {
        assert_eq!(ChiselType::Reset.to_string(), "Reset()");
    }
    #[test]
    pub(super) fn test_type_async_reset() {
        assert_eq!(ChiselType::AsyncReset.to_string(), "AsyncReset()");
    }
    #[test]
    pub(super) fn test_type_vec() {
        let t = ChiselType::Vec(4, Box::new(ChiselType::UInt(32)));
        assert_eq!(t.to_string(), "Vec(4, UInt(32.W))");
    }
    #[test]
    pub(super) fn test_type_bundle() {
        let b = ChiselType::Bundle(vec![
            ("data".to_string(), Box::new(ChiselType::UInt(8))),
            ("valid".to_string(), Box::new(ChiselType::Bool)),
        ]);
        let s = b.to_string();
        assert!(s.contains("val data = Output(UInt(8.W))"));
        assert!(s.contains("val valid = Output(Bool())"));
    }
    #[test]
    pub(super) fn test_port_input() {
        let p = ChiselPort::input("clk", ChiselType::Clock);
        assert_eq!(p.name, "clk");
        assert!(!p.is_output);
        assert_eq!(p.direction(), "Input");
    }
    #[test]
    pub(super) fn test_port_output() {
        let p = ChiselPort::output("out", ChiselType::UInt(8));
        assert_eq!(p.name, "out");
        assert!(p.is_output);
        assert_eq!(p.direction(), "Output");
    }
    #[test]
    pub(super) fn test_expr_ulit() {
        let e = ChiselExpr::ULit(42, 8);
        assert_eq!(e.to_string(), "42.U(8.W)");
    }
    #[test]
    pub(super) fn test_expr_slit() {
        let e = ChiselExpr::SLit(-1, 8);
        assert_eq!(e.to_string(), "-1.S(8.W)");
    }
    #[test]
    pub(super) fn test_expr_bool_lit() {
        assert_eq!(ChiselExpr::BoolLit(true).to_string(), "true.B");
        assert_eq!(ChiselExpr::BoolLit(false).to_string(), "false.B");
    }
    #[test]
    pub(super) fn test_expr_var() {
        let e = ChiselExpr::Var("myReg".to_string());
        assert_eq!(e.to_string(), "myReg");
    }
    #[test]
    pub(super) fn test_expr_io() {
        let e = ChiselExpr::Io("in_data".to_string());
        assert_eq!(e.to_string(), "io.in_data");
    }
    #[test]
    pub(super) fn test_expr_binop() {
        let lhs = ChiselExpr::Io("a".to_string());
        let rhs = ChiselExpr::Io("b".to_string());
        let e = ChiselExpr::BinOp(Box::new(lhs), "+".to_string(), Box::new(rhs));
        assert_eq!(e.to_string(), "(io.a + io.b)");
    }
    #[test]
    pub(super) fn test_expr_unop() {
        let inner = ChiselExpr::Io("x".to_string());
        let e = ChiselExpr::UnOp("~".to_string(), Box::new(inner));
        assert_eq!(e.to_string(), "(~(io.x))");
    }
    #[test]
    pub(super) fn test_expr_mux() {
        let e = ChiselExpr::Mux(
            Box::new(ChiselExpr::Io("sel".to_string())),
            Box::new(ChiselExpr::ULit(1, 1)),
            Box::new(ChiselExpr::ULit(0, 1)),
        );
        assert_eq!(e.to_string(), "Mux(io.sel, 1.U(1.W), 0.U(1.W))");
    }
    #[test]
    pub(super) fn test_expr_bitslice() {
        let e = ChiselExpr::BitSlice(Box::new(ChiselExpr::Var("data".to_string())), 7, 0);
        assert_eq!(e.to_string(), "data(7, 0)");
    }
    #[test]
    pub(super) fn test_expr_cat() {
        let e = ChiselExpr::Cat(vec![
            ChiselExpr::Io("msb".to_string()),
            ChiselExpr::Io("lsb".to_string()),
        ]);
        assert_eq!(e.to_string(), "Cat(io.msb, io.lsb)");
    }
    #[test]
    pub(super) fn test_expr_method_call() {
        let e = ChiselExpr::MethodCall(
            Box::new(ChiselExpr::Var("fifo".to_string())),
            "enq".to_string(),
            vec![ChiselExpr::Io("data".to_string())],
        );
        assert_eq!(e.to_string(), "fifo.enq(io.data)");
    }
    #[test]
    pub(super) fn test_connect() {
        let b = ChiselBackend::new();
        assert_eq!(b.connect("io.out", "reg_data"), "io.out := reg_data");
    }
    #[test]
    pub(super) fn test_when_stmt() {
        let b = ChiselBackend::new();
        let result = b.when_stmt("io.en", "reg_q := io.d");
        assert!(result.contains("when (io.en)"));
        assert!(result.contains("reg_q := io.d"));
    }
    #[test]
    pub(super) fn test_when_otherwise() {
        let b = ChiselBackend::new();
        let result = b.when_otherwise("io.rst", "reg_q := 0.U", "reg_q := io.d");
        assert!(result.contains("when (io.rst)"));
        assert!(result.contains(".otherwise"));
        assert!(result.contains("reg_q := 0.U"));
        assert!(result.contains("reg_q := io.d"));
    }
    #[test]
    pub(super) fn test_reg_init() {
        let b = ChiselBackend::new();
        let result = b.reg_init("counter", &ChiselType::UInt(8), "0");
        assert!(result.contains("val counter = RegInit("));
        assert!(result.contains("UInt(8.W)"));
    }
    #[test]
    pub(super) fn test_reg_no_reset() {
        let b = ChiselBackend::new();
        let result = b.reg_no_reset("data_reg", &ChiselType::UInt(32));
        assert_eq!(result, "val data_reg = Reg(UInt(32.W))");
    }
    #[test]
    pub(super) fn test_wire_decl() {
        let b = ChiselBackend::new();
        let result = b.wire_decl("tmp", &ChiselType::Bool);
        assert_eq!(result, "val tmp = Wire(Bool())");
    }
    #[test]
    pub(super) fn test_printf() {
        let b = ChiselBackend::new();
        let r1 = b.printf("val=%d", &["x"]);
        assert!(r1.contains("printf(\"val=%d\\n\", x)"));
        let r2 = b.printf("hello", &[]);
        assert!(r2.contains("printf(\"hello\\n\")"));
    }
    #[test]
    pub(super) fn test_assert_stmt() {
        let b = ChiselBackend::new();
        let result = b.assert_stmt("io.valid", "input must be valid");
        assert_eq!(result, "assert(io.valid, \"input must be valid\")");
    }
    #[test]
    pub(super) fn test_mux_expr() {
        let b = ChiselBackend::new();
        assert_eq!(b.mux_expr("sel", "a", "b"), "Mux(sel, a, b)");
    }
    #[test]
    pub(super) fn test_cat_expr() {
        let b = ChiselBackend::new();
        assert_eq!(b.cat_expr(&["msb", "lsb"]), "Cat(msb, lsb)");
    }
    #[test]
    pub(super) fn test_fill_expr() {
        let b = ChiselBackend::new();
        assert_eq!(b.fill_expr(8, "0.U(1.W)"), "Fill(8, 0.U(1.W))");
    }
    #[test]
    pub(super) fn test_instantiate() {
        let b = ChiselBackend::new();
        assert_eq!(
            b.instantiate("Adder", "u_adder"),
            "val u_adder = Module(new Adder())"
        );
    }
    #[test]
    pub(super) fn test_emit_expr() {
        let b = ChiselBackend::new();
        let e = ChiselExpr::ULit(255, 8);
        assert_eq!(b.emit_expr(&e), "255.U(8.W)");
    }
    #[test]
    pub(super) fn test_io_bundle_basic() {
        let b = ChiselBackend::new();
        let ports = vec![
            ChiselPort::input("clk", ChiselType::Clock),
            ChiselPort::input("in", ChiselType::UInt(8)),
            ChiselPort::output("out", ChiselType::UInt(8)),
        ];
        let result = b.io_bundle(&ports);
        assert!(result.contains("val io = IO(new Bundle {"));
        assert!(result.contains("val clk = Input(Clock())"));
        assert!(result.contains("val in = Input(UInt(8.W))"));
        assert!(result.contains("val out = Output(UInt(8.W))"));
    }
    #[test]
    pub(super) fn test_io_bundle_empty() {
        let b = ChiselBackend::new();
        let result = b.io_bundle(&[]);
        assert!(result.contains("val io = IO(new Bundle {"));
        assert!(result.ends_with("})"));
    }
    #[test]
    pub(super) fn test_emit_simple_passthrough() {
        let b = ChiselBackend::new();
        let mut m = ChiselModule::new("Passthrough");
        m.add_input("in", ChiselType::UInt(8));
        m.add_output("out", ChiselType::UInt(8));
        m.add_stmt(b.connect("io.out", "io.in"));
        let code = b.emit_module(&m);
        assert!(code.contains("class Passthrough extends Module"));
        assert!(code.contains("import chisel3._"));
        assert!(code.contains("val io = IO(new Bundle {"));
        assert!(code.contains("io.out := io.in"));
        assert!(code.ends_with("}\n"));
    }
    #[test]
    pub(super) fn test_emit_d_flip_flop() {
        let b = ChiselBackend::new();
        let mut m = ChiselModule::new("DFlipFlop");
        m.add_input("d", ChiselType::Bool);
        m.add_output("q", ChiselType::Bool);
        m.add_stmt(b.reg_init("reg_q", &ChiselType::Bool, "false"));
        m.add_stmt(b.connect("reg_q", "io.d"));
        m.add_stmt(b.connect("io.q", "reg_q"));
        let code = b.emit_module(&m);
        assert!(code.contains("class DFlipFlop extends Module"));
        assert!(code.contains("RegInit("));
        assert!(code.contains("io.q := reg_q"));
    }
    #[test]
    pub(super) fn test_emit_module_no_ports() {
        let b = ChiselBackend::new();
        let m = ChiselModule::new("EmptyMod");
        let code = b.emit_module(&m);
        assert!(code.contains("class EmptyMod extends Module"));
        assert!(code.ends_with("}\n"));
    }
    #[test]
    pub(super) fn test_emit_counter() {
        let b = ChiselBackend::new();
        let mut m = ChiselModule::new("Counter");
        m.add_input("en", ChiselType::Bool);
        m.add_output("count", ChiselType::UInt(8));
        m.add_stmt(b.reg_init("cnt", &ChiselType::UInt(8), "0"));
        m.add_stmt(b.when_stmt("io.en", "cnt := cnt + 1.U"));
        m.add_stmt(b.connect("io.count", "cnt"));
        let code = b.emit_module(&m);
        assert!(code.contains("class Counter extends Module"));
        assert!(code.contains("when (io.en)"));
        assert!(code.contains("cnt := cnt + 1.U"));
        assert!(code.contains("io.count := cnt"));
    }
    #[test]
    pub(super) fn test_chisel_module_add_helpers() {
        let mut m = ChiselModule::new("Test");
        m.add_input("a", ChiselType::UInt(4));
        m.add_output("b", ChiselType::UInt(4));
        m.add_stmt("val x = Wire(UInt(4.W))");
        assert_eq!(m.ports.len(), 2);
        assert_eq!(m.body.len(), 1);
    }
    #[test]
    pub(super) fn test_chisel_backend_default() {
        let b = ChiselBackend::default();
        let m = ChiselModule::new("Default");
        let code = b.emit_module(&m);
        assert!(code.contains("class Default extends Module"));
    }
}
#[cfg(test)]
mod chisel_new_tests {
    use super::*;
    #[test]
    pub(super) fn test_ready_valid_bundle_basic() {
        let rv = ChiselReadyValidBundle::new(ChiselType::UInt(32));
        let s = rv.emit_decoupled("out_port", true);
        assert!(s.contains("Decoupled"));
        assert!(s.contains("out_port"));
    }
    #[test]
    pub(super) fn test_ready_valid_bundle_with_last() {
        let rv = ChiselReadyValidBundle::new(ChiselType::UInt(8)).with_last();
        let s = rv.emit_decoupled("stream", true);
        assert!(s.contains("stream_last"));
    }
    #[test]
    pub(super) fn test_ready_valid_bundle_with_keep() {
        let rv = ChiselReadyValidBundle::new(ChiselType::UInt(64)).with_keep(8);
        let s = rv.emit_decoupled("data", true);
        assert!(s.contains("data_keep"));
        assert!(s.contains("8.W"));
    }
    #[test]
    pub(super) fn test_ready_valid_fire() {
        let rv = ChiselReadyValidBundle::new(ChiselType::UInt(32));
        let s = rv.emit_fire("ch");
        assert!(s.contains("ch_fire"));
        assert!(s.contains("ch.valid && ch.ready"));
    }
    #[test]
    pub(super) fn test_ready_valid_queue() {
        let rv = ChiselReadyValidBundle::new(ChiselType::UInt(8));
        let s = rv.emit_queue("inp", "out", 16);
        assert!(s.contains("Queue"));
        assert!(s.contains("16"));
    }
    #[test]
    pub(super) fn test_streaming_producer_emit() {
        let m = ChiselStreamingModule::producer("Src", 32).with_tlast();
        let s = m.emit();
        assert!(s.contains("class Src extends Module"));
        assert!(s.contains("tdata"));
        assert!(s.contains("tvalid"));
        assert!(s.contains("tready"));
        assert!(s.contains("tlast"));
    }
    #[test]
    pub(super) fn test_streaming_consumer_emit() {
        let m = ChiselStreamingModule::consumer("Sink", 64);
        let s = m.emit();
        assert!(s.contains("class Sink extends Module"));
        assert!(s.contains("Input(Bool())"));
    }
    #[test]
    pub(super) fn test_streaming_with_tid_and_tuser() {
        let m = ChiselStreamingModule::producer("Src2", 128)
            .with_tid(4)
            .with_tuser(8);
        let s = m.emit();
        assert!(s.contains("tid"));
        assert!(s.contains("tuser"));
        assert!(s.contains("4.W"));
        assert!(s.contains("8.W"));
    }
    #[test]
    pub(super) fn test_streaming_with_body() {
        let m = ChiselStreamingModule::producer("Gen", 32).add_stmt("val cnt = RegInit(0.U(32.W))");
        let s = m.emit();
        assert!(s.contains("val cnt"));
    }
    #[test]
    pub(super) fn test_interface_sram_master() {
        let t = ChiselInterfaceTemplate::SramPort {
            addr_bits: 10,
            data_bits: 32,
        };
        let s = t.emit_ports("sram", true);
        assert!(s.contains("sram_addr"));
        assert!(s.contains("sram_wdata"));
        assert!(s.contains("10.W"));
    }
    #[test]
    pub(super) fn test_interface_apb_master() {
        let t = ChiselInterfaceTemplate::ApbPort {
            addr_bits: 32,
            data_bits: 32,
        };
        let s = t.emit_ports("apb", true);
        assert!(s.contains("apb_paddr"));
        assert!(s.contains("apb_pready"));
        assert!(s.contains("pwrite"));
    }
    #[test]
    pub(super) fn test_interface_ahb_lite() {
        let t = ChiselInterfaceTemplate::AhbLitePort {
            addr_bits: 32,
            data_bits: 32,
        };
        let s = t.emit_ports("bus", true);
        assert!(s.contains("bus_haddr"));
        assert!(s.contains("htrans"));
    }
    #[test]
    pub(super) fn test_interface_axi4_lite() {
        let t = ChiselInterfaceTemplate::Axi4LitePort {
            addr_bits: 32,
            data_bits: 32,
        };
        let s = t.emit_ports("axi", true);
        assert!(s.contains("awvalid"));
        assert!(s.contains("arready"));
        assert!(s.contains("wstrb"));
    }
    #[test]
    pub(super) fn test_annotation_dont_touch() {
        let ann = ChiselAnnotationKind::DontTouch;
        let s = ann.scala_annotation("io.sig");
        assert!(s.contains("DontTouch"));
        assert!(s.contains("io.sig"));
    }
    #[test]
    pub(super) fn test_annotation_load_memory() {
        let ann = ChiselAnnotationKind::LoadMemoryAnnotation {
            file: "mem.hex".into(),
        };
        let s = ann.scala_annotation("mem_module");
        assert!(s.contains("loadMemoryFromFile"));
        assert!(s.contains("mem.hex"));
    }
    #[test]
    pub(super) fn test_annotation_inline_instance() {
        let ann = ChiselAnnotationKind::InlineInstance;
        let s = ann.scala_annotation("sub");
        assert!(s.contains("Inline"));
    }
    #[test]
    pub(super) fn test_pipeline_stage_chain() {
        let chain = ChiselPipelineRegisterChain::new("0")
            .stage(PipelineStage::new("s0", ChiselType::UInt(32)).with_valid())
            .stage(
                PipelineStage::new("s1", ChiselType::UInt(32))
                    .with_valid()
                    .with_stall(),
            );
        let s = chain.emit_registers();
        assert!(s.contains("s0"));
        assert!(s.contains("s1"));
        assert!(s.contains("s0_valid"));
        assert!(s.contains("s1_stall"));
    }
    #[test]
    pub(super) fn test_pipeline_stage_count() {
        let chain = ChiselPipelineRegisterChain::new("0")
            .stage(PipelineStage::new("a", ChiselType::Bool))
            .stage(PipelineStage::new("b", ChiselType::Bool))
            .stage(PipelineStage::new("c", ChiselType::Bool));
        assert_eq!(chain.stage_count(), 3);
    }
    #[test]
    pub(super) fn test_sram_wrapper_single_port_emit() {
        let sram = ChiselSRAMWrapper::single_port("RegFile", 1024, 64);
        let s = sram.emit();
        assert!(s.contains("class RegFile extends Module"));
        assert!(s.contains("SyncReadMem(1024"));
        assert!(s.contains("64.W"));
        assert!(s.contains("wen"));
    }
    #[test]
    pub(super) fn test_sram_wrapper_addr_width() {
        let sram = ChiselSRAMWrapper::single_port("M", 256, 32);
        assert_eq!(sram.addr_width(), 8);
    }
    #[test]
    pub(super) fn test_sram_wrapper_with_mask() {
        let sram = ChiselSRAMWrapper::single_port("M", 64, 32).with_mask(8);
        assert_eq!(sram.mask_width(), 4);
        let s = sram.emit();
        assert!(s.contains("wmask"));
    }
    #[test]
    pub(super) fn test_sram_wrapper_sdp() {
        let sram = ChiselSRAMWrapper::simple_dual_port("Sdp", 512, 32);
        let s = sram.emit();
        assert!(s.contains("raddr"));
        assert!(s.contains("rdata"));
    }
    #[test]
    pub(super) fn test_sram_wrapper_pipeline_read() {
        let sram = ChiselSRAMWrapper::single_port("PL", 64, 8).with_pipeline_read();
        let s = sram.emit();
        assert!(s.contains("raddr_r"));
        assert!(s.contains("RegNext"));
    }
    #[test]
    pub(super) fn test_backend_dont_care() {
        let b = ChiselBackend::new();
        let s = b.dont_care("io.out");
        assert!(s.contains("DontCare"));
        assert!(s.contains("io.out"));
    }
    #[test]
    pub(super) fn test_backend_irrevocable_port() {
        let b = ChiselBackend::new();
        let s = b.irrevocable_port("out", &ChiselType::UInt(8), true);
        assert!(s.contains("Irrevocable"));
        assert!(s.contains("out"));
    }
    #[test]
    pub(super) fn test_backend_comb_rom() {
        let b = ChiselBackend::new();
        let s = b.comb_rom("lut", &ChiselType::UInt(8), &["0", "1", "2", "255"]);
        assert!(s.contains("VecInit"));
        assert!(s.contains("255.U"));
    }
    #[test]
    pub(super) fn test_backend_mux1h() {
        let b = ChiselBackend::new();
        let s = b.mux1h("sel", &[("sel(0)", "a"), ("sel(1)", "b")]);
        assert!(s.contains("Mux1H"));
        assert!(s.contains("sel(0)"));
    }
    #[test]
    pub(super) fn test_backend_log2_ceil() {
        let b = ChiselBackend::new();
        assert_eq!(b.log2_ceil(1), 1);
        assert_eq!(b.log2_ceil(8), 3);
        assert_eq!(b.log2_ceil(9), 4);
    }
    #[test]
    pub(super) fn test_backend_log2_floor() {
        let b = ChiselBackend::new();
        assert_eq!(b.log2_floor(8), 3);
        assert_eq!(b.log2_floor(9), 3);
    }
    #[test]
    pub(super) fn test_backend_is_pow2() {
        let b = ChiselBackend::new();
        assert!(b.is_pow2(1));
        assert!(b.is_pow2(64));
        assert!(!b.is_pow2(0));
        assert!(!b.is_pow2(3));
    }
    #[test]
    pub(super) fn test_backend_cover_stmt() {
        let b = ChiselBackend::new();
        let s = b.cover_stmt("io.valid", "valid_active");
        assert!(s.contains("cover"));
        assert!(s.contains("io.valid"));
    }
    #[test]
    pub(super) fn test_backend_assume_stmt() {
        let b = ChiselBackend::new();
        let s = b.assume_stmt("io.en");
        assert!(s.contains("assume"));
    }
    #[test]
    pub(super) fn test_backend_cat() {
        let b = ChiselBackend::new();
        let s = b.cat(&["a", "b", "c"]);
        assert!(s.contains("Cat(a, b, c)"));
    }
    #[test]
    pub(super) fn test_backend_popcount() {
        let b = ChiselBackend::new();
        let s = b.popcount("mask");
        assert!(s.contains("PopCount(mask)"));
    }
    #[test]
    pub(super) fn test_backend_oh_to_uint() {
        let b = ChiselBackend::new();
        let s = b.oh_to_uint("oh_sig");
        assert!(s.contains("OHToUInt(oh_sig)"));
    }
    #[test]
    pub(super) fn test_backend_mux_case() {
        let b = ChiselBackend::new();
        let s = b.mux_case("0.U", &[("a === 1.U", "x"), ("a === 2.U", "y")]);
        assert!(s.contains("MuxCase"));
        assert!(s.contains("a === 1.U"));
    }
    #[test]
    pub(super) fn test_backend_shift_register() {
        let b = ChiselBackend::new();
        let s = b.shift_register("io.in", 4, "0");
        assert!(s.contains("ShiftRegister"));
        assert!(s.contains("io.in"));
        assert!(s.contains(", 4,"));
    }
    #[test]
    pub(super) fn test_backend_rr_arbiter() {
        let b = ChiselBackend::new();
        let s = b.round_robin_arbiter("arb", &ChiselType::UInt(8), 4);
        assert!(s.contains("RRArbiter"));
        assert!(s.contains("arb"));
    }
    #[test]
    pub(super) fn test_backend_priority_arbiter() {
        let b = ChiselBackend::new();
        let s = b.priority_arbiter("parb", &ChiselType::Bool, 2);
        assert!(s.contains("Arbiter"));
    }
    #[test]
    pub(super) fn test_backend_queue_module() {
        let b = ChiselBackend::new();
        let s = b.queue_module("fifo", &ChiselType::UInt(32), 8);
        assert!(s.contains("Queue"));
        assert!(s.contains("fifo"));
        assert!(s.contains(", 8)"));
    }
    #[test]
    pub(super) fn test_backend_when_chain() {
        let b = ChiselBackend::new();
        let s = b.when_chain(&[("a", "x := 1.U"), ("b", "x := 2.U")], Some("x := 0.U"));
        assert!(s.contains("when (a)"));
        assert!(s.contains("elsewhen (b)"));
        assert!(s.contains("otherwise"));
    }
    #[test]
    pub(super) fn test_backend_counter() {
        let b = ChiselBackend::new();
        let s = b.counter("cyc", 99, "true.B");
        assert!(s.contains("Counter"));
        assert!(s.contains("cyc_count"));
    }
    #[test]
    pub(super) fn test_backend_reset_sync() {
        let b = ChiselBackend::new();
        let s = b.reset_sync("rst", "io.async_rst");
        assert!(s.contains("rst_sync"));
        assert!(s.contains("asAsyncReset"));
    }
    #[test]
    pub(super) fn test_backend_cdc_comment() {
        let b = ChiselBackend::new();
        let s = b.cdc_handshake_comment("clk_a", "clk_b");
        assert!(s.contains("CDC"));
        assert!(s.contains("clk_a"));
    }
    #[test]
    pub(super) fn test_backend_blackbox_stub_no_params() {
        let b = ChiselBackend::new();
        let s = b.blackbox_stub("MyIp", &[]);
        assert!(s.contains("BlackBox"));
        assert!(s.contains("class MyIp"));
    }
    #[test]
    pub(super) fn test_backend_blackbox_stub_with_params() {
        let b = ChiselBackend::new();
        let s = b.blackbox_stub("ParamIp", &[("WIDTH", "32"), ("DEPTH", "16")]);
        assert!(s.contains("Map("));
        assert!(s.contains("WIDTH"));
        assert!(s.contains("32"));
    }
    #[test]
    pub(super) fn test_backend_fill() {
        let b = ChiselBackend::new();
        let s = b.fill(8, "io.bit");
        assert!(s.contains("Fill(8, io.bit)"));
    }
    #[test]
    pub(super) fn test_backend_reverse() {
        let b = ChiselBackend::new();
        let s = b.reverse("io.data");
        assert!(s.contains("Reverse(io.data)"));
    }
    #[test]
    pub(super) fn test_backend_uint_to_oh() {
        let b = ChiselBackend::new();
        let s = b.uint_to_oh("io.sel", 8);
        assert!(s.contains("UIntToOH"));
        assert!(s.contains("io.sel"));
    }
    #[test]
    pub(super) fn test_pipeline_stage_no_valid() {
        let s = PipelineStage::new("s", ChiselType::UInt(8));
        assert!(!s.has_valid);
        assert!(!s.has_stall);
    }
    #[test]
    pub(super) fn test_streaming_direction_producer() {
        let m = ChiselStreamingModule::producer("P", 8);
        assert_eq!(m.direction, StreamDirection::Producer);
    }
    #[test]
    pub(super) fn test_streaming_direction_consumer() {
        let m = ChiselStreamingModule::consumer("C", 8);
        assert_eq!(m.direction, StreamDirection::Consumer);
    }
    #[test]
    pub(super) fn test_sram_mask_granularity_byte() {
        let sram = ChiselSRAMWrapper::single_port("M", 64, 64).with_mask(8);
        assert_eq!(sram.mask_width(), 8);
    }
    #[test]
    pub(super) fn test_interface_template_sram_slave() {
        let t = ChiselInterfaceTemplate::SramPort {
            addr_bits: 12,
            data_bits: 16,
        };
        let s = t.emit_ports("mem", false);
        assert!(s.contains("mem_addr"));
        assert!(s.contains("12.W"));
    }
}
#[cfg(test)]
mod Chisel_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = ChiselPassConfig::new("test_pass", ChiselPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = ChiselPassStats::new();
        stats.record_run(10, 100, 3);
        stats.record_run(20, 200, 5);
        assert_eq!(stats.total_runs, 2);
        assert!((stats.average_changes_per_run() - 15.0).abs() < 0.01);
        assert!((stats.success_rate() - 1.0).abs() < 0.01);
        let s = stats.format_summary();
        assert!(s.contains("Runs: 2/2"));
    }
    #[test]
    pub(super) fn test_pass_registry() {
        let mut reg = ChiselPassRegistry::new();
        reg.register(ChiselPassConfig::new("pass_a", ChiselPassPhase::Analysis));
        reg.register(ChiselPassConfig::new("pass_b", ChiselPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = ChiselAnalysisCache::new(10);
        cache.insert("key1".to_string(), vec![1, 2, 3]);
        assert!(cache.get("key1").is_some());
        assert!(cache.get("key2").is_none());
        assert!((cache.hit_rate() - 0.5).abs() < 0.01);
        cache.invalidate("key1");
        assert!(!cache.entries["key1"].valid);
        assert_eq!(cache.size(), 1);
    }
    #[test]
    pub(super) fn test_worklist() {
        let mut wl = ChiselWorklist::new();
        assert!(wl.push(1));
        assert!(wl.push(2));
        assert!(!wl.push(1));
        assert_eq!(wl.len(), 2);
        assert_eq!(wl.pop(), Some(1));
        assert!(!wl.contains(1));
        assert!(wl.contains(2));
    }
    #[test]
    pub(super) fn test_dominator_tree() {
        let mut dt = ChiselDominatorTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 3));
        assert!(!dt.dominates(2, 3));
        assert!(dt.dominates(3, 3));
    }
    #[test]
    pub(super) fn test_liveness() {
        let mut liveness = ChiselLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(ChiselConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(ChiselConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(ChiselConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            ChiselConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(ChiselConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = ChiselDepGraph::new();
        g.add_dep(1, 2);
        g.add_dep(2, 3);
        g.add_dep(1, 3);
        assert_eq!(g.dependencies_of(2), vec![1]);
        let topo = g.topological_sort();
        assert_eq!(topo.len(), 3);
        assert!(!g.has_cycle());
        let pos: std::collections::HashMap<u32, usize> =
            topo.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        assert!(pos[&1] < pos[&2]);
        assert!(pos[&1] < pos[&3]);
        assert!(pos[&2] < pos[&3]);
    }
}
