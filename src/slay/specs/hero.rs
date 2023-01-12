use crate::slay::showdown::consequences::{Condition, RollConsequence, RollConsequences};

use crate::slay::abilities::heros::VictimDraws;
use crate::slay::tasks::core::destroy::{DestroyCardTask, DestroyModifiersDestination};
use crate::slay::tasks::core::pull::PullFromTask;
use crate::slay::tasks::core::steal::{StealCardFromTask, StealTask};
use crate::slay::tasks::heros::mimimeow::Mimimeow;
use crate::slay::tasks::heros::slippery_paws::SlipperyPaws;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::task_params::TaskParamName;
use crate::slay::tasks::tasks::immediate::{OfferPlayImmediately, PlayImmediatelyFilter};
use crate::slay::tasks::tasks::params::{
	ChooseCardFromPlayerParameterTask, ChoosePlayerParameterTask, ClearParamsTask,
};
use enum_iterator::Sequence;

#[derive(Debug, Clone)]
pub struct HeroAbility {
	pub condition: Condition,
	pub ability: HeroAbilityType,
}

impl HeroAbility {
	pub fn to_consequences(&self) -> RollConsequences {
		RollConsequences {
			success: RollConsequence {
				condition: self.condition.to_owned(),
				tasks: self.ability.create_tasks(),
			},
			loss: None,
		}
	}
}

// Some renaming is appropriate...
#[derive(Clone, Debug, Sequence, PartialEq, Copy)]
pub enum HeroAbilityType {
	PlunderingPuma,
	SlipperyPaws,
	SmoothMimimeow,
	Meowzio,
	Shurikitty,
	KitNapper,
	SilentShadow,
	SlyPickings,
	HolyCurselifter,
	IronResolve,
	CalmingVoice,
	VibrantGlow,
	MightyBlade,
	RadiantHorn,
	GuidingLight,
	WiseShield,
	QiBear,
	PanChucks,
	HeavyBear,
	BadAxe,
	ToughTeddy,
	BearClaw,
	FuryKnuckle,
	BearyWise,
	Hook,
	Wildshot,
	SeriousGrey,
	WilyRed,
	QuickDraw,
	LookieRookie,
	Bullseye,
	SharpFox,
	FuzzyCheeks,
	Peanut,
	NappingNibbles,
	TipsyTootie,
	MellowDee,
	LuckBucky,
	DodgyDealer,
	GreedyCheeks,
	Fluffy,
	Wiggles,
	Spooky,
	Snowball,
	Buttons,
	BunBun,
	Hopper,
	Whiskers,
}

impl HeroAbilityType {
	pub fn create_tasks(&self) -> Vec<Box<dyn PlayerTask>> {
		match self {
			HeroAbilityType::PlunderingPuma => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::PlayerToPullFrom,
					"Choose a player to pull from.",
				),
				PullFromTask::create(TaskParamName::PlayerToPullFrom),
				PullFromTask::create(TaskParamName::PlayerToPullFrom),
				VictimDraws::create(TaskParamName::PlayerToPullFrom),
				ClearParamsTask::create(),
			],
			HeroAbilityType::SlipperyPaws => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::SlipperyPawsVictim,
					"Choose a player to pull 2 cards from, you will have to discard one of them.",
				),
				PullFromTask::record_pulled(
					TaskParamName::SlipperyPawsVictim,
					Some(TaskParamName::SlipperyPawsVictimPulledCard1),
				),
				PullFromTask::record_pulled(
					TaskParamName::SlipperyPawsVictim,
					Some(TaskParamName::SlipperyPawsVictimPulledCard2),
				),
				SlipperyPaws::create(),
				ClearParamsTask::create(),
			],
			HeroAbilityType::SmoothMimimeow => vec![Mimimeow::create(), ClearParamsTask::create()],
			HeroAbilityType::Meowzio => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::MeowzioVictim,
					"Choose a player to steal and pull from.",
				),
				PullFromTask::create(TaskParamName::MeowzioVictim),
				ChooseCardFromPlayerParameterTask::from_party(
					TaskParamName::MeowzioVictim,
					TaskParamName::MeowzioCard,
					"Which hero card would you like to steal?",
				),
				StealCardFromTask::create(TaskParamName::MeowzioVictim, TaskParamName::MeowzioCard),
				ClearParamsTask::create(),
			],
			HeroAbilityType::Shurikitty => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::PlayerToDestroy,
					"to destroy a hero card (Shurikitty)",
				),
				ChooseCardFromPlayerParameterTask::from_party(
					TaskParamName::PlayerToDestroy,
					TaskParamName::CardToSteal,
					"Which hero card would you like to destroy?",
				),
				DestroyCardTask::create(
					TaskParamName::PlayerToDestroy,
					TaskParamName::CardToSteal,
					DestroyModifiersDestination::Myself,
				),
				ClearParamsTask::create(),
			],
			HeroAbilityType::KitNapper => vec![StealTask::create()],
			HeroAbilityType::SilentShadow => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::SilentShadowVictim,
					"Who's hand do you want to see?",
				),
				ChooseCardFromPlayerParameterTask::from_party(
					TaskParamName::SilentShadowVictim,
					TaskParamName::SilentShadowCard,
					"Which hero card would you like to take?",
				),
				// TODO
				ClearParamsTask::create(),
			],
			HeroAbilityType::SlyPickings => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::SlyPickinsVictim,
					"Sly Pickings: Who do you want to steal from?",
				),
				PullFromTask::record_pulled(
					TaskParamName::SlyPickinsVictim,
					Some(TaskParamName::SlyPickinsCard),
				),
				OfferPlayImmediately::create(
					TaskParamName::SlyPickinsCard,
					PlayImmediatelyFilter::IsMagic,
				),
				ClearParamsTask::create(),
			],
			HeroAbilityType::HolyCurselifter => vec![],
			HeroAbilityType::IronResolve => vec![],
			HeroAbilityType::CalmingVoice => vec![],
			HeroAbilityType::VibrantGlow => vec![],
			HeroAbilityType::MightyBlade => vec![],
			HeroAbilityType::RadiantHorn => vec![],
			HeroAbilityType::GuidingLight => vec![],
			HeroAbilityType::WiseShield => vec![],
			HeroAbilityType::QiBear => vec![],
			HeroAbilityType::PanChucks => vec![],
			HeroAbilityType::HeavyBear => vec![],
			HeroAbilityType::BadAxe => vec![],
			HeroAbilityType::ToughTeddy => vec![],
			HeroAbilityType::BearClaw => vec![],
			HeroAbilityType::FuryKnuckle => vec![],
			HeroAbilityType::BearyWise => vec![],
			HeroAbilityType::Hook => vec![],
			HeroAbilityType::Wildshot => vec![],
			HeroAbilityType::SeriousGrey => vec![],
			HeroAbilityType::WilyRed => vec![],
			HeroAbilityType::QuickDraw => vec![],
			HeroAbilityType::LookieRookie => vec![],
			HeroAbilityType::Bullseye => vec![],
			HeroAbilityType::SharpFox => vec![],
			HeroAbilityType::FuzzyCheeks => vec![],
			HeroAbilityType::Peanut => vec![],
			HeroAbilityType::NappingNibbles => vec![],
			HeroAbilityType::TipsyTootie => vec![],
			HeroAbilityType::MellowDee => vec![],
			HeroAbilityType::LuckBucky => vec![],
			HeroAbilityType::DodgyDealer => vec![],
			HeroAbilityType::GreedyCheeks => vec![],
			HeroAbilityType::Fluffy => vec![],
			HeroAbilityType::Wiggles => vec![],
			HeroAbilityType::Spooky => vec![],
			HeroAbilityType::Snowball => vec![],
			HeroAbilityType::Buttons => vec![],
			HeroAbilityType::BunBun => vec![],
			HeroAbilityType::Hopper => vec![],
			HeroAbilityType::Whiskers => vec![],
		}
	}
}
