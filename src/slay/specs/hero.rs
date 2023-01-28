use enum_iterator::Sequence;

use crate::slay::abilities::heros::VictimDraws;
use crate::slay::modifiers::ModifierOrigin;
use crate::slay::modifiers::PlayerModifier;
use crate::slay::showdown::consequences::Condition;
use crate::slay::showdown::consequences::RollConsequence;
use crate::slay::showdown::consequences::RollConsequences;
use crate::slay::specification::HeroType;
use crate::slay::tasks::core::destroy::DestroyCardTask;
use crate::slay::tasks::core::destroy::DestroyModifiersDestination;
use crate::slay::tasks::core::destroy::DestroyTask;
use crate::slay::tasks::core::discard::Discard;
use crate::slay::tasks::core::discard::DiscardFromParam;
use crate::slay::tasks::core::discard::PlayersWithHeroTypeDiscard;
use crate::slay::tasks::core::draw::DrawTask;
use crate::slay::tasks::core::pull::PullFromTask;
use crate::slay::tasks::core::sacrifice::Sacrifice;
use crate::slay::tasks::core::steal::StealCardFromTask;
use crate::slay::tasks::core::steal::StealTask;
use crate::slay::tasks::heros::greedy_cheeks::GreedyCheeks;
use crate::slay::tasks::heros::mimimeow::Mimimeow;
use crate::slay::tasks::heros::pan_chucks::PanChucksDestroy;
use crate::slay::tasks::heros::qi_bear::QiBear;
use crate::slay::tasks::heros::quick_draw_style::QuickDrawStyle;
use crate::slay::tasks::heros::slippery_paws::SlipperyPaws;
use crate::slay::tasks::heros::spooky::Spooky;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::task_params::TaskParamName;
use crate::slay::tasks::tasks::immediate::OfferPlayImmediately;
use crate::slay::tasks::tasks::immediate::PlayImmediatelyFilter;
use crate::slay::tasks::tasks::params::ChooseCardFromPlayerParameterTask;
use crate::slay::tasks::tasks::params::ChoosePlayerParameterTask;
use crate::slay::tasks::tasks::params::ClearParamsTask;
use crate::slay::tasks::tasks::place_hero::PlaceHero;
use crate::slay::tasks::tasks::pull_again::PullAgain;
use crate::slay::tasks::tasks::receive_modifier::ReceiveModifier;
use crate::slay::tasks::tasks::return_modifiers::ReturnModifierTask;
use crate::slay::tasks::tasks::search_discard::SearchDiscard;
use crate::slay::tasks::tasks::trade_hands::TradeHands;

use super::cards::card_type::SlayCardSpec;

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
// Call this HeroCard
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
	pub fn label(&self) -> &'static str {
		// Kind of backwards: that method should call this one...
		SlayCardSpec::HeroCard(*self).label()
	}

	pub fn hero_type(&self) -> HeroType {
		let ret = match self {
			HeroAbilityType::PlunderingPuma
			| HeroAbilityType::SlipperyPaws
			| HeroAbilityType::SmoothMimimeow
			| HeroAbilityType::Meowzio
			| HeroAbilityType::Shurikitty
			| HeroAbilityType::KitNapper
			| HeroAbilityType::SilentShadow
			| HeroAbilityType::SlyPickings => HeroType::Thief,
			HeroAbilityType::HolyCurselifter
			| HeroAbilityType::IronResolve
			| HeroAbilityType::CalmingVoice
			| HeroAbilityType::VibrantGlow
			| HeroAbilityType::MightyBlade
			| HeroAbilityType::RadiantHorn
			| HeroAbilityType::GuidingLight
			| HeroAbilityType::WiseShield => HeroType::Gaurdian,
			HeroAbilityType::QiBear
			| HeroAbilityType::PanChucks
			| HeroAbilityType::HeavyBear
			| HeroAbilityType::BadAxe
			| HeroAbilityType::ToughTeddy
			| HeroAbilityType::BearClaw
			| HeroAbilityType::FuryKnuckle
			| HeroAbilityType::BearyWise => HeroType::Fighter,
			HeroAbilityType::Hook
			| HeroAbilityType::Wildshot
			| HeroAbilityType::SeriousGrey
			| HeroAbilityType::WilyRed
			| HeroAbilityType::QuickDraw
			| HeroAbilityType::LookieRookie
			| HeroAbilityType::Bullseye
			| HeroAbilityType::SharpFox => HeroType::Ranger,
			HeroAbilityType::FuzzyCheeks
			| HeroAbilityType::Peanut
			| HeroAbilityType::NappingNibbles
			| HeroAbilityType::TipsyTootie
			| HeroAbilityType::MellowDee
			| HeroAbilityType::LuckBucky
			| HeroAbilityType::DodgyDealer
			| HeroAbilityType::GreedyCheeks => HeroType::Bard,
			HeroAbilityType::Fluffy
			| HeroAbilityType::Wiggles
			| HeroAbilityType::Spooky
			| HeroAbilityType::Snowball
			| HeroAbilityType::Buttons
			| HeroAbilityType::BunBun
			| HeroAbilityType::Hopper
			| HeroAbilityType::Whiskers => HeroType::Wizard,
		};
		if Some(ret)
			!= SlayCardSpec::HeroCard(*self)
				.get_card_spec_creation()
				.get_unmodified_hero_type()
		{
			log::info!("Mismatch hero type {:?}", self);
			unreachable!()
		}
		ret
	}

	pub fn to_consequences(&self) -> RollConsequences {
		if let Some(ability) = SlayCardSpec::HeroCard(*self)
			.get_card_spec_creation()
			.hero_ability
		{
			return ability.to_consequences();
		}
		{
			unreachable!();
		}
	}

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
					TaskParamName::SlipperyPawsVictimPulledCard1,
				),
				PullFromTask::record_pulled(
					TaskParamName::SlipperyPawsVictim,
					TaskParamName::SlipperyPawsVictimPulledCard2,
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
					TaskParamName::SlyPickinsCard,
				),
				OfferPlayImmediately::create(
					TaskParamName::SlyPickinsCard,
					PlayImmediatelyFilter::IsMagic,
				),
				ClearParamsTask::create(),
			],
			HeroAbilityType::HolyCurselifter => vec![ReturnModifierTask::return_everyones()],
			HeroAbilityType::IronResolve => vec![ReceiveModifier::for_this_turn(
				PlayerModifier::NoCardsCanBeChallenged,
				ModifierOrigin::FromHeroAbility,
			)],
			HeroAbilityType::CalmingVoice => vec![ReceiveModifier::until_next_turn(
				PlayerModifier::NoCardsCanBeStolen,
				ModifierOrigin::FromHeroAbility,
			)],
			HeroAbilityType::VibrantGlow => vec![ReceiveModifier::for_this_turn(
				PlayerModifier::AddToAllRolls(5),
				ModifierOrigin::FromHeroAbility,
			)],
			HeroAbilityType::MightyBlade => vec![ReceiveModifier::until_next_turn(
				PlayerModifier::NoCardsCanBeDestroyed,
				ModifierOrigin::FromHeroAbility,
			)],
			HeroAbilityType::RadiantHorn => vec![SearchDiscard::for_modifiers()],
			HeroAbilityType::GuidingLight => vec![SearchDiscard::for_hero()],
			HeroAbilityType::WiseShield => vec![ReceiveModifier::for_this_turn(
				PlayerModifier::AddToAllRolls(3),
				ModifierOrigin::FromHeroAbility,
			)],
			HeroAbilityType::QiBear => vec![QiBear::create()],
			HeroAbilityType::PanChucks => vec![
				DrawTask::into_param(TaskParamName::PanChuckFirstCard),
				DrawTask::into_param(TaskParamName::PanChuckSecondCard),
				PanChucksDestroy::create(),
				ClearParamsTask::create(),
			],
			HeroAbilityType::HeavyBear => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::HeavyBearVictim,
					"who has to discard 2 cards (Heavy bear)",
				),
				DiscardFromParam::create(2, TaskParamName::HeavyBearVictim),
				ClearParamsTask::create(),
			],
			HeroAbilityType::BadAxe => vec![DestroyTask::create()],
			HeroAbilityType::ToughTeddy => vec![PlayersWithHeroTypeDiscard::create(HeroType::Fighter)],
			HeroAbilityType::BearClaw => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::BearClawVictim,
					"BearClaw: Who would you like to pull from?",
				),
				PullFromTask::record_pulled(TaskParamName::BearClawVictim, TaskParamName::BearClawCard),
				PullAgain::create(
					TaskParamName::BearClawVictim,
					TaskParamName::BearClawCard,
					PlayImmediatelyFilter::IsHero,
				),
				ClearParamsTask::create(),
			],
			HeroAbilityType::FuryKnuckle => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::FuryKnuckleVictim,
					"Fury Knuckle: Who would you like to pull from?",
				),
				PullFromTask::record_pulled(
					TaskParamName::FuryKnuckleVictim,
					TaskParamName::FuryKnuckleCard,
				),
				PullAgain::create(
					TaskParamName::FuryKnuckleVictim,
					TaskParamName::FuryKnuckleCard,
					PlayImmediatelyFilter::IsChallenge,
				),
				ClearParamsTask::create(),
			],
			HeroAbilityType::BearyWise => vec![], ////////////////////////////////////
			HeroAbilityType::Hook => vec![
				// Hook::create(),
				// OfferPlayImmediately::with_an_extra_task(),
				// There is similar logic in... immediately.rs
				DrawTask::create(1),
			],
			HeroAbilityType::Wildshot => vec![DrawTask::create(3), Discard::create(1)],
			HeroAbilityType::SeriousGrey => vec![DestroyTask::create(), DrawTask::create(1)],
			HeroAbilityType::WilyRed => vec![DrawTask::until(7)],
			HeroAbilityType::QuickDraw => vec![
				DrawTask::into_param(TaskParamName::QuickDrawCard1),
				DrawTask::into_param(TaskParamName::QuickDrawCard2),
				QuickDrawStyle::create(
					TaskParamName::QuickDrawCard1,
					TaskParamName::QuickDrawCard2,
					PlayImmediatelyFilter::IsItem,
				),
			],
			HeroAbilityType::LookieRookie => vec![SearchDiscard::for_item()],
			HeroAbilityType::Bullseye => vec![
				// Bullseye::create(),
			],
			HeroAbilityType::SharpFox => vec![], /////////////////////////////////////
			HeroAbilityType::FuzzyCheeks => vec![DrawTask::create(1), PlaceHero::create()], //////////////////////////////////
			HeroAbilityType::Peanut => vec![DrawTask::create(2)],
			HeroAbilityType::NappingNibbles => vec![/* This one actually is empty. */],
			HeroAbilityType::TipsyTootie => vec![], //////////////////////////////////
			HeroAbilityType::MellowDee => vec![
				DrawTask::into_param(TaskParamName::MellowDeeVictim),
				ClearParamsTask::create(),
			],
			HeroAbilityType::LuckBucky => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::LuckBuckyVictim,
					"LuckBucky: Who would you like to pull from?",
				),
				PullFromTask::record_pulled(TaskParamName::LuckBuckyVictim, TaskParamName::LuckBuckyCard),
				OfferPlayImmediately::create(TaskParamName::LuckBuckyCard, PlayImmediatelyFilter::IsHero),
				ClearParamsTask::create(),
			],
			HeroAbilityType::DodgyDealer => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::DodgyDealerVictim,
					"Dodger Dealer: Who do you want to trade hands with?",
				),
				TradeHands::create(TaskParamName::DodgyDealerVictim),
				ClearParamsTask::create(),
			],
			HeroAbilityType::GreedyCheeks => vec![GreedyCheeks::create()],
			HeroAbilityType::Fluffy => vec![DestroyTask::create(), DestroyTask::create()],
			HeroAbilityType::Wiggles => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::WigglesVictim,
					"Wiggles: Who do you want to steal a hero card from?",
				),
				ChooseCardFromPlayerParameterTask::from_party(
					TaskParamName::PlayerToStealFrom,
					TaskParamName::CardToSteal,
					"Which hero card would you like to steal?",
				),
				StealCardFromTask::create(TaskParamName::WigglesVictim, TaskParamName::WigglesCard),
				OfferPlayImmediately::create(TaskParamName::WigglesCard, PlayImmediatelyFilter::None),
				ClearParamsTask::create(),
			],
			HeroAbilityType::Spooky => vec![Spooky::create()],
			HeroAbilityType::Snowball => vec![
				DrawTask::into_param(TaskParamName::SnowballCard),
				OfferPlayImmediately::with_an_extra_task(
					TaskParamName::SnowballCard,
					PlayImmediatelyFilter::IsMagic,
					DrawTask::create(1),
				),
				ClearParamsTask::create(),
			],
			HeroAbilityType::Buttons => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::ButtonsVictim,
					"Buttons: Who would you like to pull from?",
				),
				PullFromTask::record_pulled(TaskParamName::ButtonsVictim, TaskParamName::ButtonsCard),
				OfferPlayImmediately::create(TaskParamName::ButtonsCard, PlayImmediatelyFilter::IsMagic),
				ClearParamsTask::create(),
			],
			HeroAbilityType::BunBun => vec![SearchDiscard::for_magic()],
			HeroAbilityType::Hopper => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::HopperVictim,
					"Hopper: Who do you want to sacrifice",
				),
				Sacrifice::from_param(TaskParamName::HopperVictim),
				ClearParamsTask::create(),
			],
			HeroAbilityType::Whiskers => vec![StealTask::create(), DestroyTask::create()],
		}
	}
}
